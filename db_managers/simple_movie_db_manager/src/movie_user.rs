use std::collections::HashMap;

use crate::schema::*;

use db_manager::User;
use crate::movie_item::MovieItem;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "users"]
pub struct QueryableUser {
    pub id: i32,
    pub username: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
}

#[derive(Debug,Clone)]
pub struct MovieUser {
    pub id: i32,
    pub name: String,
    pub ratings: HashMap<i32, f64>
}

impl User<MovieItem> for MovieUser {
    type ID = i32;

    fn id(&self) -> i32 {
        self.id as i32
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn data(&self) -> HashMap<String, String> {
        HashMap::new()
    }
    fn ratings(&self) -> HashMap<i32, f64> {
        self.ratings.clone()
    }
}