use std::collections::HashMap;

use crate::schema::*;

use db_manager::Item;

#[derive(diesel::Queryable)]
pub struct QueryableItem {
    pub id: i32,
    pub title: String,
    pub genres: String,
}

#[derive(Insertable)]
#[table_name="movies"]
pub struct NewMovie {
    pub id: i32,
    pub title: String,
    pub genres: String,
}

#[derive(Debug)]
pub struct MovieItem {
    pub id: i32,
    pub title: String,
    pub extra_data: HashMap<String, String>
}

impl Item for MovieItem {
    type ID = i32;
    
    fn id(&self) -> i32 {
        self.id
    }
    fn name(&self) -> String {
        self.title.clone()
    }
    fn data(&self) -> HashMap<String, String> {
        self.extra_data.clone()
    }
}

impl MovieItem {
    pub fn create(id:i32, title:String, genres:String) -> MovieItem {
        let mut extra_data:HashMap<String,String> = HashMap::new();

        extra_data.insert(String::from("Genres"), genres);

        MovieItem{id: id, title: title, extra_data: extra_data}
    }
}