use std::collections::HashMap;

#[derive(diesel::Queryable)]
pub struct QueryableRating {
    pub id: i32,
    pub user_id: i32,
    pub movie_id: i32,
    pub rating: f64
}