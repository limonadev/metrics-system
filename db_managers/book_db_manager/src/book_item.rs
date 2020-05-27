use std::collections::HashMap;

use crate::schema::*;

use db_manager::Item;

#[derive(diesel::Queryable)]
pub struct QueryableItem {
    pub id: String,
    pub title: String,
    pub author: String,
    pub pub_year: String,
    pub publisher: String
}

#[derive(Insertable)]
#[table_name="books"]
pub struct NewBook {
    pub id: String,
    pub title: String,
    pub author: String,
    pub pub_year: String,
    pub publisher: String
}

#[derive(Clone)]
pub struct BookItem {
    pub id: String,
    pub title: String,
    pub extra_data: HashMap<String, String>
}

impl core::fmt::Debug for BookItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BookItem")
            .field("id", &self.id)
            .field("title", &self.title)
            .finish()
    }
    
}

impl Item for BookItem {
    type ID = String;
    
    fn id(&self) -> String {
        self.id.clone()
    }
    fn name(&self) -> String {
        self.title.clone()
    }
    fn data(&self) -> HashMap<String, String> {
        self.extra_data.clone()
    }
}

impl BookItem {
    pub fn create(id:String, title:String, author:String, pub_year:String, publisher:String) -> BookItem {
        let mut extra_data:HashMap<String,String> = HashMap::new();
        extra_data.insert(String::from("Author"), author);
        extra_data.insert(String::from("Publication Year"), pub_year);
        extra_data.insert(String::from("Publisher"), publisher);

        BookItem{id: id, title: title, extra_data: extra_data}
    }
}