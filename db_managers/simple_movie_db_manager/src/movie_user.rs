use std::collections::HashMap;

use crate::schema::*;

use db_manager::User;

#[derive(diesel::Queryable)]
pub struct QueryableUser {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
}

pub struct MovieUser {
    pub id: i32,
    pub name: String,
    pub ratings: HashMap<u64, f64>
}

impl User for MovieUser {
    fn id(&self) -> u64 {
        self.id as u64
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn data(&self) -> HashMap<String, String> {
        HashMap::new()
    }
    fn ratings(&self) -> HashMap<u64, f64> {
        self.ratings.clone()
    }
}