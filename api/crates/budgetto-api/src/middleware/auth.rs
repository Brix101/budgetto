use std::task::{Context, Poll};

use axum::{
    http::{
        header::{AUTHORIZATION, COOKIE},
        HeaderValue, Request,
    },
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::cookie::Cookie;
use budgetto_domain::{
    sessions::requests::NewAccessTokenRequest, users::responses::ReAuthResponse,
};
use budgetto_infrastructure::service_register::ServiceRegister;
use futures::executor::block_on;
use tower::{Layer, Service};

#[derive(Clone)]
pub struct AuthenticationLayer {
    service_register: ServiceRegister,
}

impl From<ServiceRegister> for AuthenticationLayer {
    fn from(value: ServiceRegister) -> Self {
        Self {
            service_register: value.clone(),
        }
    }
}

impl<S> Layer<S> for AuthenticationLayer {
    type Service = AuthenticationService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthenticationService {
            inner,
            services_register: self.service_register.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AuthenticationService<S> {
    inner: S,
    services_register: ServiceRegister,
}

impl<S, B> Service<Request<B>> for AuthenticationService<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        // Do something with `self.state`.
        //
        // See `axum::RequestExt` for how to run extractors directly from
        // a `Request`.
        let services = self.services_register.clone();
        let headers = req.headers();
        let authorization_header = headers
            .get(AUTHORIZATION)
            .and_then(|header| header.to_str().ok());
        let cookie_header = headers.get(COOKIE).and_then(|header| header.to_str().ok());

        let re_auth: ReAuthResponse = if let Some(auth_header) = authorization_header {
            let tokenized_value: Vec<_> = auth_header.split(' ').collect();

            if tokenized_value.len() != 2 || tokenized_value.get(1).is_none() {
                ReAuthResponse::default();
            }

            let token_value = tokenized_value.into_iter().nth(1).unwrap();
            let verified_user = services.token_service.verify_access_token(token_value);

            if verified_user.is_ok() {
                let user = verified_user.unwrap();
                let auth_user = ReAuthResponse {
                    user: Some(user),
                    access_token: None,
                };
                auth_user
            } else if verified_user
                .err()
                .unwrap()
                .to_string()
                .contains("ExpiredSignature")
            {
                if let Some(cookie_result) = cookie_header {
                    let cookie_value = Cookie::parse(cookie_result).unwrap();
                    let refresh_token_value = cookie_value.value();

                    let token_request = NewAccessTokenRequest {
                        refresh_token: Some(refresh_token_value.to_string()),
                    };
                    let requested_token =
                        block_on(services.sessions.refresh_access_token(token_request));

                    if requested_token.is_ok() {
                        requested_token.unwrap()
                    } else {
                        ReAuthResponse::default()
                    }
                } else {
                    ReAuthResponse::default()
                }
            } else {
                ReAuthResponse::default()
            }
        } else {
            ReAuthResponse::default()
        };

        req.extensions_mut().insert(re_auth);
        self.inner.call(req)
    }
}

pub async fn token_refresher<B>(mut request: Request<B>, next: Next<B>) -> impl IntoResponse {
    let extensions_mut = request.extensions_mut();
    let re_auth_response = extensions_mut.get::<ReAuthResponse>();

    let x_access_token: Option<String> = if let Some(re_auth) = re_auth_response {
        re_auth.access_token.clone()
    } else {
        None
    };

    let mut response = next.run(request).await;

    if x_access_token.is_some() {
        response.headers_mut().insert(
            "x-access-token",
            HeaderValue::from_str(&x_access_token.unwrap()).unwrap(),
        );
    }
    response
}
