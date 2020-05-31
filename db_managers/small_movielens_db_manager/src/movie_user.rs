use std::collections::HashMap;

use crate::schema::*;

use db_manager::User;
use crate::movie_item::SMovieLensItem;

#[derive(Identifiable, Queryable)]
#[table_name = "users"]
pub struct QueryableUser {
    pub id: i32,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub id: i32,
}

#[derive(Debug,Clone)]
pub struct SMovieLensUser {
    pub id: i32,
    pub ratings: HashMap<i32, f64>
}

impl User<SMovieLensItem> for SMovieLensUser {
    type ID = i32;

    fn id(&self) -> i32 {
        self.id
    }
    fn name(&self) -> String {
        String::from("")
    }
    fn data(&self) -> HashMap<String, String> {
        HashMap::new()
    }
    fn ratings(&self) -> HashMap<i32, f64> {
        self.ratings.clone()
    }
}