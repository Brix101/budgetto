// use async_trait::async_trait;
// use axum::extract::FromRequest;
// use axum::http::{Request, StatusCode};
// use axum::{BoxError, Json};
// use serde::de::DeserializeOwned;
// use validator::Validate;

// use crate::core::errors::CustomError;
// /// use this to encapsulate fields that require validation
// #[derive(Debug, Clone, Copy, Default)]
// pub struct ValidationExtractor<T>(pub T);

// #[async_trait]
// impl<S, T, B> FromRequest<S, B> for ValidationExtractor<T>
// where
//     B: Send + 'static + http_body::Body,
//     S: Send + Sync,
//     T: DeserializeOwned + Validate + 'static,
//     B::Data: Send,
//     B::Error: Into<BoxError>,
// {
//     type Rejection = CustomError;

//     async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
//         let Json(value) = Json::<T>::from_request(request).await?;
//         value.validate()?;
//         Ok(ValidationExtractor(value))
//     }
// }
