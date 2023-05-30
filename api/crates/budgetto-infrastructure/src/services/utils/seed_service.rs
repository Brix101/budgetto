use budgetto_domain::{
    categories::requests::CreateCategoryDto,
    users::{
        requests::{SignInUserDto, SignUpUserDto},
        UserDto,
    },
};
use lazy_static::lazy_static;
use tracing::info;

use budgetto_core::{
    categories::service::DynCategoriesService, errors::AppResult, users::service::DynUsersService,
};

use crate::service_register::ServiceRegister;

lazy_static! {
    static ref TEST_USER_1_NAME: &'static str = "testuser1";
    static ref TEST_USER_1_EMAIL: &'static str = "testuser1@gmail.com";
    static ref TEST_USER_1_PASSWORD: &'static str = "password";
    static ref TEST_USER_2_NAME: &'static str = "testuser2";
    static ref TEST_USER_2_EMAIL: &'static str = "testuser2@gmail.com";
    static ref TEST_USER_2_PASSWORD: &'static str = "password";
    static ref TEST_USER_3_NAME: &'static str = "testuser3";
    static ref TEST_USER_3_EMAIL: &'static str = "testuser3@gmail.com";
    static ref TEST_USER_3_PASSWORD: &'static str = "password";
}

pub struct SeedService {
    user_services: DynUsersService,
    category_services: DynCategoriesService,
}

impl SeedService {
    pub fn new(services: ServiceRegister) -> Self {
        Self {
            user_services: services.users,
            category_services: services.categories,
        }
    }

    pub async fn seed(&self) -> AppResult<()> {
        // assume that if we have an active user in the users table, data has been seeded
        let seed_data_exists = self
            .user_services
            .signin(SignInUserDto {
                email: Some(String::from(*TEST_USER_1_EMAIL)),
                password: Some(String::from(*TEST_USER_1_PASSWORD)),
                user_agent: None,
            })
            .await
            .is_ok();

        if seed_data_exists {
            info!("data has already been seeded, bypassing test data setup");
            return Ok(());
        }
        info!("seeding users...");
        self.create_user(*TEST_USER_1_NAME, *TEST_USER_1_EMAIL, *TEST_USER_1_PASSWORD)
            .await?;

        self.create_user(*TEST_USER_2_NAME, *TEST_USER_2_EMAIL, *TEST_USER_2_PASSWORD)
            .await?;

        self.create_user(*TEST_USER_3_NAME, *TEST_USER_3_EMAIL, *TEST_USER_3_PASSWORD)
            .await?;

        info!("users created, seeding categories...");

        self.category_services
            .create(
                CreateCategoryDto {
                    name: Some(String::from("Housing")),
                    note: Some(String::from("This category can include mortgage or rent payments, property taxes, homeowners or renters insurance, repairs and maintenance, and utilities.")),
                    user_id:None
                },
            )
            .await?;

        self.category_services
            .create(
                CreateCategoryDto {
                    name: Some(String::from("Transportation")),
                    note: Some(String::from("This category can include car payments, gas, car insurance, maintenance and repairs, and public transportation expenses.")),
                    user_id:None
                },
            )
            .await?;

        self.category_services
            .create(CreateCategoryDto {
                name: Some(String::from("Food")),
                note: Some(String::from(
                    "This category can include groceries, dining out, and snacks.",
                )),
                user_id: None,
            })
            .await?;

        self.category_services
            .create(
                CreateCategoryDto {
                    name: Some(String::from("Personal Care")),
                    note: Some(String::from("This category can include items such as haircuts, personal grooming products, and gym memberships.")),
                    user_id:None
                },
            )
            .await?;

        self.category_services
            .create(
                CreateCategoryDto {
                    name: Some(String::from("Entertainment")),
                    note: Some(String::from("This category can include expenses for movies, concerts, hobbies, and vacations.")),
                    user_id:None
                },
            )
            .await?;

        self.category_services
            .create(
                CreateCategoryDto {
                    name: Some(String::from("Debt Payments")),
                    note: Some(String::from("This category can include payments towards credit card debt, student loans, or other debts.")),
                    user_id:None
                },
            )
            .await?;

        self.category_services
            .create(
                CreateCategoryDto {
                    name: Some(String::from("Savings")),
                    note: Some(String::from("This category can include savings towards retirement, emergency funds, or other financial goals.")),
                    user_id:None
                },
            )
            .await?;

        self.category_services
            .create(
                CreateCategoryDto {
                    name: Some(String::from("Utilities")),
                    note: Some(String::from("This category can include expenses for electricity, gas, water, internet, and phone.")),
                    user_id:None
                },
            )
            .await?;

        self.category_services
            .create(
                CreateCategoryDto {
                    name: Some(String::from("Health Care")),
                    note: Some(String::from("This category can include expenses for health insurance, doctor visits, prescriptions, and other medical expenses.")),
                    user_id:None
                },
            )
            .await?;

        info!("seed ran successfully!");
        Ok(())
    }

    async fn create_user(
        &self,
        name: &'static str,
        email: &'static str,
        password: &'static str,
    ) -> AppResult<UserDto> {
        self.user_services
            .signup(SignUpUserDto {
                name: Some(String::from(name)),
                email: Some(String::from(email)),
                password: Some(String::from(password)),
            })
            .await
    }
}
