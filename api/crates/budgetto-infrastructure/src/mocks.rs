use budgetto_core::categories::repository::MockCategoriesRepository;

pub struct CategoriesServiceTestFixture {
    pub mock_categories_repository: MockCategoriesRepository,
}

impl CategoriesServiceTestFixture {
    pub fn new() -> Self {
        Self {
            mock_categories_repository: MockCategoriesRepository::new(),
        }
    }
}

impl Default for CategoriesServiceTestFixture {
    fn default() -> Self {
        CategoriesServiceTestFixture::new()
    }
}
