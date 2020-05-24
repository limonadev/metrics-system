use std::collections::HashMap;

use crate::schema::*;

#[derive(diesel::Queryable)]
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