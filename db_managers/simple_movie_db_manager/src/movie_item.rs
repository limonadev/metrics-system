use std::collections::HashMap;

use crate::schema::*;

use db_manager::Item;

#[derive(diesel::Queryable)]
pub struct QueryableItem {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable)]
#[table_name="movies"]
pub struct NewMovie {
    pub title: String,
}

pub struct MovieItem {
    pub id: i32,
    pub name: String,
    pub ratings: HashMap<u64, f64>
}

impl Item for MovieItem {
    fn id(&self) -> u64 {
        self.id as u64
    }
    fn name(&self) -> String {
        self.name.clone()
    }
    fn data(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}