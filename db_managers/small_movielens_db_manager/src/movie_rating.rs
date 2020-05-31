use crate::schema::*;

use crate::movie_user::QueryableUser;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(QueryableUser, foreign_key="user_id")]
#[table_name = "ratings"]
pub struct QueryableRating {
    pub id: i32,
    pub user_id: i32,
    pub movie_id: i32,
    pub rating: f64
}

#[derive(Insertable)]
#[table_name="ratings"]
pub struct NewRating {
    pub user_id: i32,
    pub movie_id: i32,
    pub rating: f64
}