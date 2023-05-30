use axum::extract::{Json, Path};
use axum::routing::{get, put};
use axum::{Extension, Router};
use tracing::info;

use budgetto_core::errors::{AppResult, Error};
use budgetto_domain::users::requests::UpdateUserDto;
use budgetto_domain::users::responses::{UserAuthenicationResponse, UserDataResponse};
use budgetto_infrastructure::service_register::ServiceRegister;
use uuid::Uuid;

use crate::extractors::required_authentication_extractor::RequiredAuthentication;

pub struct UserRouter;

impl UserRouter {
    pub fn app() -> Router {
        Router::new()
            .route("/", put(Self::update_user_endpoint))
            .route("/:id", get(Self::get_user_data))
    }

    pub async fn update_user_endpoint(
        RequiredAuthentication(user_id): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
        Json(mut request): Json<UpdateUserDto>,
    ) -> AppResult<Json<UserAuthenicationResponse>> {
        info!("recieved request to update user {:?}", user_id);

        request.id = Some(user_id);
        let updated_user = services.users.updated(request).await?;

        Ok(Json(UserAuthenicationResponse { user: updated_user }))
    }

    pub async fn get_user_data(
        Path(id): Path<Uuid>,
        RequiredAuthentication(user_id): RequiredAuthentication,
        Extension(services): Extension<ServiceRegister>,
    ) -> AppResult<Json<UserDataResponse>> {
        info!("recieved request to get user {:?} data", user_id);

        if id != user_id {
            return Err(Error::Forbidden);
        }

        let user = services.users.find_by_id(user_id).await?;
        let accounts = services.accounts.find_many(user_id).await?;
        let budgets = services.budgets.find_many(user_id).await?;
        let categories = services.categories.find_many(user_id).await?;
        let transactions = services.transactions.find_many(user_id).await?;

        Ok(Json(UserDataResponse {
            user,
            accounts,
            budgets,
            categories,
            transactions,
        }))
    }
}
