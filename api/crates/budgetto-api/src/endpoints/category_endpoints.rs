use axum::extract::{Json, Path, Query};
use axum::routing::{delete, get, post, put};
use axum::{Extension, Router};
use budgetto_infrastructure::service_register::ServiceRegister;
use tracing::info;
use uuid::Uuid;

use budgetto_core::errors::AppResult;
use budgetto_domain::categories::requests::{CreateCategoryDto, QueryCategory, UpdateCategoryDto};
use budgetto_domain::categories::responses::CategoriesResponse;
use budgetto_domain::categories::CategoryDto;

use crate::extractors::required_authentication_extractor::RequiredAuthentication;
use crate::extractors::validation_extractor::ValidationExtractor;

pub struct CategoryRouter;

impl CategoryRouter {
    pub fn app() -> Router {
        Router::new()
            .route("/", get(Self::get_categories))
            .route("/", post(Self::create_category))
            .route("/:id", get(Self::get_category))
            .route("/:id", put(Self::update_category))
            .route("/:id", delete(Self::delete_category))
    }

    pub async fn get_categories(
        query_params: Query<QueryCategory>,
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
    ) -> AppResult<Json<CategoriesResponse>> {
        info!("received request to get current user categories");

        if let Some(id) = query_params.id {
            // return this function if the query params has value
            let category = services.categories.find_by_id(id, user.id).await?;

            return Ok(Json(CategoriesResponse {
                categories: vec![category],
            }));
        }

        let categories = services.categories.find_many(user.id).await?;

        Ok(Json(CategoriesResponse { categories }))
    }

    pub async fn get_category(
        Path(id): Path<Uuid>,
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
    ) -> AppResult<Json<CategoryDto>> {
        info!("recieved request to get category {:?}", id);

        let category = services.categories.find_by_id(id, user.id).await?;

        Ok(Json(category))
    }
    pub async fn create_category(
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
        ValidationExtractor(mut request): ValidationExtractor<CreateCategoryDto>,
    ) -> AppResult<Json<CategoryDto>> {
        info!("received request to create category");

        request.user_id = Some(user.id);
        let new_category = services.categories.create(request).await?;

        Ok(Json(new_category))
    }

    pub async fn update_category(
        Path(id): Path<Uuid>,
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
        Json(mut request): Json<UpdateCategoryDto>,
    ) -> AppResult<Json<CategoryDto>> {
        info!("recieved request to update category {:?}", id);

        request.id = Some(id);
        request.user_id = Some(user.id);

        let updated_category = services.categories.updated(request).await?;

        Ok(Json(updated_category))
    }

    pub async fn delete_category(
        Path(id): Path<Uuid>,
        RequiredAuthentication(user): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
    ) -> AppResult<()> {
        info!("recieved request to remove category {:?}", id);

        services.categories.delete(id, user.id).await?;

        Ok(())
    }
}
