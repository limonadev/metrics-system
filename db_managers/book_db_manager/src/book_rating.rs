use crate::schema::*;

#[derive(diesel::Queryable)]
pub struct QueryableRating {
    pub id: i32,
    pub user_id: i32,
    pub book_id: String,
    pub rating: f64
}

#[derive(Insertable)]
#[table_name="ratings"]
pub struct NewRating {
    pub user_id: i32,
    pub book_id: String,
    pub rating: f64
}