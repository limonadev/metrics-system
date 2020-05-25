use std::collections::HashMap;

use crate::schema::*;

use db_manager::User;
use crate::book_item::BookItem;

#[derive(diesel::Queryable)]
pub struct QueryableUser {
    pub id: i32,
    pub city: String,
    pub age: Option<i32>
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub id: i32,
    pub city: String,
    pub age: Option<i32>
}

#[derive(Debug)]
pub struct BookUser {
    pub id: i32,
    pub extra_data: HashMap<String, String>,
    pub ratings: HashMap<String, f64>
}

impl User<BookItem> for BookUser {
    type ID = i32;

    fn id(&self) -> i32 {
        self.id
    }
    fn name(&self) -> String {
        String::from("")
    }
    fn data(&self) -> HashMap<String, String> {
        self.extra_data.clone()
    }
    fn ratings(&self) -> HashMap<String, f64> {
        self.ratings.clone()
    }
}

impl BookUser {
    pub fn create(id:i32, ratings:HashMap<String, f64>, city:String, age:Option<i32>) -> BookUser {
        let mut extra_data:HashMap<String,String> = HashMap::new();
        extra_data.insert(String::from("City"), city);

        if let Some(age) = age {
            extra_data.insert(String::from("Age"), age.to_string());
        }

        BookUser{id: id, ratings: ratings, extra_data: extra_data}
    }
}