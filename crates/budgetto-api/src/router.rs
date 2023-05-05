use std::future::ready;
use std::net::{Ipv4Addr, SocketAddr};
use std::time::{Duration, Instant};

use anyhow::{Context, Ok};
use axum::extract::MatchedPath;
use axum::http::header::{AUTHORIZATION, COOKIE};
use axum::http::{header, HeaderValue, Method, Request};
use axum::middleware::{self, Next};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Extension;
use axum::{error_handling::HandleErrorLayer, http::StatusCode, BoxError, Json, Router};
use axum_extra::extract::cookie::Cookie;
use budgetto_domain::sessions::requests::NewAccessTokenRequest;
use lazy_static::lazy_static;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder};
use serde_json::json;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

use budgetto_domain::PingResponse;
use budgetto_infrastructure::service_register::ServiceRegister;

use crate::endpoints;

lazy_static! {
    static ref HTTP_TIMEOUT: u64 = 30;
    static ref EXPONENTIAL_SECONDS: &'static [f64] =
        &[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,];
}

pub struct ApplicationController;

impl ApplicationController {
    pub async fn serve(
        port: u16,
        cors_origin: &str,
        service_register: ServiceRegister,
    ) -> anyhow::Result<()> {
        let recorder_handle = PrometheusBuilder::new()
            .set_buckets_for_metric(
                Matcher::Full(String::from("http_requests_duration_seconds")),
                *EXPONENTIAL_SECONDS,
            )
            .context("could not setup buckets for metrics, verify matchers are correct")?
            .install_recorder()
            .context("could not install metrics recorder")?;

        let allowed_origin = cors_origin
            .split(",")
            .map(|origin| origin.trim().parse().unwrap())
            .collect::<Vec<HeaderValue>>();

        let cors_layer = CorsLayer::new()
            .allow_credentials(true)
            .allow_headers(vec![
                header::ACCEPT,
                header::ACCEPT_LANGUAGE,
                header::AUTHORIZATION,
                header::CONTENT_LANGUAGE,
                header::CONTENT_TYPE,
                header::ORIGIN,
            ])
            .allow_methods(vec![
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
            ])
            .allow_origin(allowed_origin)
            .max_age(Duration::from_secs(60 * 60));

        let service_builder = ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(HandleErrorLayer::new(Self::handle_timeout_error))
            .layer(BufferLayer::new(1024))
            .layer(Extension(service_register.clone()))
            .layer(RateLimitLayer::new(5, Duration::from_secs(1)))
            .timeout(Duration::from_secs(*HTTP_TIMEOUT));

        let router = Router::new()
            .nest("/api/v1", endpoints::app())
            .route("/api/v1/ping", get(Self::ping))
            .route("/metrics", get(move || ready(recorder_handle.render())))
            .layer(service_builder)
            .layer(cors_layer)
            .route_layer(middleware::from_fn(Self::token_refresher))
            .route_layer(middleware::from_fn(Self::track_metrics))
            .layer(Extension(service_register.clone()));

        let router = router.fallback(Self::handle_404);
        let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));

        info!("🚀 Server has launched on https://{addr}");

        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .with_graceful_shutdown(Self::shutdown_signal())
            .await
            .context("error while starting API server")?;

        Ok(())
    }
    async fn token_refresher<B>(
        Extension(services): Extension<ServiceRegister>,
        mut request: Request<B>,
        next: Next<B>,
    ) -> impl IntoResponse {
        let headers = request.headers_mut();
        let header_cookie = headers.get(COOKIE);
        let header_auth = headers.get(AUTHORIZATION);
        let auth_result =
            if let (Some(cookie_result), Some(auth_result)) = (header_cookie, header_auth) {
                let header_cookie_value = cookie_result.to_str().unwrap();
                let header_auth_value = auth_result.to_str().unwrap();

                let cookie_value = Cookie::parse(header_cookie_value).unwrap();

                let refresh_token_value = cookie_value.value();

                let token_value = header_auth_value.replace("Bearer ", "");
                let verify_result = services.token_service.verify_access_token(&token_value);

                if verify_result.is_ok() {
                    Ok(token_value)
                } else {
                    let token_request = NewAccessTokenRequest {
                        refresh_token: Some(refresh_token_value.to_string()),
                    };
                    let new_access_token = services
                        .sessions
                        .refresh_access_token(token_request)
                        .await
                        .unwrap();
                    Ok(new_access_token.access_token)
                }
            } else {
                Ok(String::new())
            };
        let new_token = auth_result.unwrap();

        headers.insert("user", "test".parse().unwrap());
        if !new_token.is_empty() {
            headers.insert(
                AUTHORIZATION,
                format!("Bearer {}", new_token.clone()).parse().unwrap(),
            );
        }
        let mut response = next.run(request).await;

        if !new_token.is_empty() {
            response
                .headers_mut()
                .insert("x-access-token", new_token.clone().parse().unwrap());
        }
        response
    }
    /// Adds a custom handler for tower's `TimeoutLayer`, see https://docs.rs/axum/latest/axum/middleware/index.html#commonly-used-middleware.
    async fn handle_timeout_error(err: BoxError) -> (StatusCode, Json<serde_json::Value>) {
        if err.is::<tower::timeout::error::Elapsed>() {
            (
                StatusCode::REQUEST_TIMEOUT,
                Json(json!({
                    "error":
                        format!(
                            "request took longer than the configured {} second timeout",
                            *HTTP_TIMEOUT
                        )
                })),
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("unhandled internal error: {}", err)
                })),
            )
        }
    }

    async fn track_metrics<B>(request: Request<B>, next: Next<B>) -> impl IntoResponse {
        let path = if let Some(matched_path) = request.extensions().get::<MatchedPath>() {
            matched_path.as_str().to_owned()
        } else {
            request.uri().path().to_owned()
        };

        let start = Instant::now();
        let method = request.method().clone();
        let response = next.run(request).await;
        let latency = start.elapsed().as_secs_f64();
        let status = response.status().as_u16().to_string();

        let labels = [
            ("method", method.to_string()),
            ("path", path),
            ("status", status),
        ];

        metrics::increment_counter!("http_requests_total", &labels);
        metrics::histogram!("http_requests_duration_seconds", latency, &labels);

        response
    }

    /// Tokio signal handler that will wait for a user to press CTRL+C.
    /// We use this in our hyper `Server` method `with_graceful_shutdown`.
    async fn shutdown_signal() {
        tokio::signal::ctrl_c()
            .await
            .expect("expect tokio signal ctrl-c");
        println!("signal shutdown");
    }

    async fn handle_404() -> impl IntoResponse {
        (
            StatusCode::NOT_FOUND,
            axum::response::Json(serde_json::json!({
            "errors":{
            "message": vec!(String::from("The requested resource does not exist on this server!")),}
            })),
        )
    }
    async fn ping() -> Json<PingResponse> {
        info!("received ping request");
        Json(PingResponse::default())
    }
}
