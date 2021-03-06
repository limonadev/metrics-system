use crate::schema::*;
use crate::book_user::QueryableUser;

use crate::schema::{users, ratings};

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(QueryableUser, foreign_key="user_id")]
#[table_name = "ratings"]
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