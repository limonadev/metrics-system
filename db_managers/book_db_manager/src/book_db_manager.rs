use std::collections::HashMap;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use db_manager::DBManager;

use crate::schema::{users, books, ratings};
use crate::{book_user::{BookUser, QueryableUser}, book_item::{BookItem, QueryableItem}, book_rating::{QueryableRating}};

pub struct BookDBManager {
    connector:PgConnection
}

impl DBManager<BookUser, BookItem> for BookDBManager {
    fn connect_to(url: &str) -> Self {
        let connector = PgConnection::establish(url).expect("Failed connection to database. Maybe the URL?");
        BookDBManager{connector: connector}
    }

    fn get_user_by_name(&self, name: &str) -> Vec<BookUser> {
        vec![]
    }

    fn get_user_by_id(&self, uid: i32) -> Vec<BookUser> {
        let query_result = users::table.filter(users::id.eq(uid))
            .load::<QueryableUser>(&self.connector)
            .expect("Failed query of user with the uid specified");

        if query_result.is_empty() {
            return Vec::new();
        }

        let selected_user = &query_result[0];

        let query_result = ratings::table.filter(ratings::user_id.eq(selected_user.id))
            .load::<QueryableRating>(&self.connector)
            .expect(&format!("Failed query of ratings of the user {}", selected_user.id));

        let mut user_ratings = HashMap::new();
        for rating in &query_result {
            user_ratings.insert(rating.book_id.clone(), rating.rating);
        }

        vec![BookUser::create(selected_user.id, user_ratings, selected_user.city.clone(), selected_user.age)]
    }

    fn get_item_by_name(&self, name: &str) -> Vec<BookItem> {
        let query_result = books::table.filter(books::title.eq(name))
            .load::<QueryableItem>(&self.connector)
            .expect("Failed query of book with the given title");

        let mut result = Vec::new();

        for book in &query_result {
            result.push(
                BookItem::create(
                    book.id.clone(),
                    book.title.clone(),
                    book.author.clone(),
                    book.pub_year.clone(),
                    book.publisher.clone()
                )
            );
        }

        result
    }

    fn get_item_by_id(&self, uid: String) -> Vec<BookItem> {
        let query_result = books::table.filter(books::id.eq(uid))
            .load::<QueryableItem>(&self.connector)
            .expect("Failed query of book with the given uid");

        if query_result.is_empty() {
            return Vec::new();
        }

        let book = &query_result[0];

        vec![
            BookItem::create(
                book.id.clone(),
                book.title.clone(),
                book.author.clone(),
                book.pub_year.clone(),
                book.publisher.clone()
            )
        ]
    }

    fn get_all_users(&self) -> Vec<BookUser> {
        let query_result = users::table
            .load::<QueryableUser>(&self.connector)
            .expect("Failed query of all users");

        let mut result = Vec::new();

        for selected_user in &query_result {
            let query_result = ratings::table.filter(ratings::user_id.eq(selected_user.id))
                .load::<QueryableRating>(&self.connector)
                .expect(&format!("Failed query of ratings of the user {}", selected_user.id));
            
            let mut user_ratings = HashMap::new();
            for rating in &query_result {
                user_ratings.insert(rating.book_id.clone(), rating.rating);
            }

            result.push(BookUser::create(selected_user.id, user_ratings, selected_user.city.clone(), selected_user.age));
        }

        result
    }

    fn get_all_ratings(&self) -> HashMap<i32, HashMap<String, f64>> {
        let query_result = ratings::table
            .load::<QueryableRating>(&self.connector)
            .expect("Failed to fetch all ratings");

        let mut result = HashMap::new();
        for rating in &query_result {
            if !result.contains_key(&rating.user_id) {
                result.insert(rating.user_id, HashMap::new());
            }
            let user_ratings = result.get_mut(&rating.user_id).unwrap();
            user_ratings.insert(rating.book_id.clone(), rating.rating);
        }
        result
    }
}