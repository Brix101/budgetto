use std::sync::Arc;

use budgetto_api::{
    database::category::repository::{Category, DynCategoriesRepository},
    mocks::CategoriesServiceTestFixture,
    server::{
        dtos::category_dto::CategoryCreateDto,
        services::category_services::{CategoriesService, CategoriesServiceTrait},
    },
};
use mockall::predicate::*;

#[tokio::test]
async fn return_success_when_downstream_services_succeedand_category_exists() {
    // arrange
    let mut fixture = CategoriesServiceTestFixture::default();

    fixture
        .mock_repository
        .expect_create_category()
        .with(eq(1), eq(String::from("stub category")))
        .times(1)
        .return_once(move |_, _| Ok(Category::default()));

    fixture
        .mock_repository
        .expect_get_category_by_id()
        .with(eq(1))
        .times(1)
        .return_once(move |_| Ok(Some(Category::default())));

    fixture
        .mock_repository
        .expect_get_categories()
        .with(eq(1))
        .times(1)
        .return_once(move |_| Ok(vec![Category::default()]));

    fixture
        .mock_repository
        .expect_update_category()
        .with(eq(1), eq(String::from("stub category")))
        .times(1)
        .return_once(move |_, _| Ok(Category::default()));

    fixture
        .mock_repository
        .expect_delete_category()
        .with(eq(1))
        .times(1)
        .return_once(move |_| Ok(()));

    let categories_service =
        CategoriesService::new(Arc::new(fixture.mock_repository) as DynCategoriesRepository);

    // act
    let response = categories_service
        .create_category(1, CategoryCreateDto::new_stub())
        .await;

    // assert
    assert!(response.is_ok());
}
