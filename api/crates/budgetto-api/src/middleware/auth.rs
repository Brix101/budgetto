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
use budgetto_domain::users::AuthenticationDto;
use budgetto_infrastructure::service_register::ServiceRegister;
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

        let auth: AuthenticationDto = match authorization_header.as_deref() {
            Some(auth_header) => {
                let tokenized_value: Vec<_> = auth_header.split(' ').collect();

                if tokenized_value.len() != 2 || tokenized_value.get(1).is_none() {
                    AuthenticationDto::default();
                }

                let token_value = tokenized_value.into_iter().nth(1).unwrap();
                let user = services
                    .token_service
                    .verify_access_token(token_value)
                    .map_err(|err| {
                        if err.to_string().contains("ExpiredSignature") {
                            AuthenticationDto::expired()
                        } else {
                            AuthenticationDto::default()
                        }
                    });

                match user {
                    Ok(user) => AuthenticationDto::into_auth(user),
                    Err(err) => err,
                }
            }
            None => AuthenticationDto::default(),
        };

        req.extensions_mut().insert(auth);
        self.inner.call(req)
    }
}

pub async fn token_refresher<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let headers = req.headers().clone();
    let extensions_mut = req.extensions_mut();

    let auth_extension = extensions_mut.get::<AuthenticationDto>().clone();
    let services_extension = extensions_mut.get::<ServiceRegister>().unwrap().clone();

    let x_access_token: Option<String> = match auth_extension {
        Some(auth) => {
            if auth.is_expired {
                let cookie_header = headers.get(COOKIE).and_then(|header| header.to_str().ok());

                match cookie_header {
                    Some(cookie_result) => {
                        let cookie_value = Cookie::parse(cookie_result).unwrap();
                        let token_value = cookie_value.value();

                        let requested_token = services_extension
                            .sessions
                            .refresh_access_token(token_value)
                            .await;

                        match requested_token {
                            Ok(res) => {
                                extensions_mut.insert(AuthenticationDto::into_auth(res.user));

                                Some(res.access_token)
                            }
                            Err(_) => None,
                        }
                    }
                    None => None,
                }
            } else {
                None
            }
        }
        None => None,
    };

    let mut response = next.run(req).await;

    if x_access_token.is_some() {
        response.headers_mut().insert(
            "x-access-token",
            HeaderValue::from_str(&x_access_token.unwrap()).unwrap(),
        );
    }
    response
}
