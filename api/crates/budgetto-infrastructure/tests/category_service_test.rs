use std::sync::Arc;

use budgetto_core::categories::{
    repository::{CategoryEntity, DynCategoriesRepository},
    service::CategoriesService,
};
use budgetto_infrastructure::{
    mocks::CategoriesServiceTestFixture, services::categories_service::BudgettoCategoriesService,
};
use mockall::predicate::*;
use uuid::uuid;

#[tokio::test]
async fn return_success_when_downstream_services_succeed_and_category_exists() {
    // arrange
    let mut fixture = CategoriesServiceTestFixture::default();

    fixture
        .mock_categories_repository
        .expect_find_by_id()
        .with(eq(uuid!("e0edf148-717b-491c-ad58-402adf892313")))
        .times(1)
        .return_once(move |_| Ok(Some(CategoryEntity::default())));

    let categories_service = BudgettoCategoriesService::new(Arc::new(
        fixture.mock_categories_repository,
    ) as DynCategoriesRepository);

    // act
    let response = categories_service
        .find_many(uuid!("f3f898aa-ffa3-4b58-91b0-612a1c801a5e"))
        .await;

    // assert
    assert!(response.is_ok());
}
