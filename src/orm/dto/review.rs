#[derive(Debug)]
pub struct Review {
    pub review_id: i32,
    pub user_id: i32,
    pub game_id: i32,
    pub rating: i32,
    pub review_comment: String,
}
