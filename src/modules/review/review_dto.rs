use poem_openapi::Object;
use uuid::Uuid;

#[derive(Debug, Object)]
pub struct ReviewDto {
    pub id: Uuid,
    pub rating: i32,
    pub title: String,
    pub comment: String,
    pub created_at: String,
    pub user_id: Uuid,
    pub business_id: Uuid,
    pub listing_id: Uuid,
}

#[derive(Debug, Object)]
pub struct CreateReviewDto {
    #[oai(validator(minimum(value = "1"), maximum(value = "5")))]
    pub rating: i32,
    #[oai(validator(min_length = 3, max_length = 120))]
    pub title: String,
    #[oai(validator(min_length = 3, max_length = 1000))]
    pub comment: String,
}

#[derive(Debug, Object)]
pub struct UpdateReviewDto {
    #[oai(validator(minimum(value = "1"), maximum(value = "5")))]
    pub rating: Option<i32>,
    #[oai(validator(min_length = 3, max_length = 120))]
    pub title: Option<String>,
    #[oai(validator(min_length = 3, max_length = 1000))]
    pub comment: Option<String>,
}
