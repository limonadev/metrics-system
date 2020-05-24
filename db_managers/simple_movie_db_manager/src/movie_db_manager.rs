use std::collections::HashMap;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use db_manager::DBManager;

use crate::schema::{users, ratings};
use crate::{movie_user::{MovieUser, QueryableUser}, movie_item::{MovieItem, QueryableItem}, movie_rating::{QueryableRating}};

pub struct MovieDBManager {
    connector:PgConnection
}

impl DBManager<MovieUser, MovieItem> for MovieDBManager {
    fn connect_to(url: &str) -> Self {
        let connector = PgConnection::establish(url).expect("Failed connection to database. Maybe the URL?");
        MovieDBManager{connector: connector}
    }

    fn get_user_by_name(&self, name: &str) -> Vec<MovieUser> {
        let query_result = users::table.filter(users::username.eq(name))
            .load::<QueryableUser>(&self.connector)
            .expect("Failed query of users with the username specified");

        let mut result = Vec::new();

        for selected_user in &query_result {
            let query_result = ratings::table.filter(ratings::user_id.eq(selected_user.id))
                .load::<QueryableRating>(&self.connector)
                .expect(&format!("Failed query of ratings of the user {}", selected_user.username));
            
            let mut user_ratings = HashMap::new();
            for rating in &query_result {
                user_ratings.insert(rating.movie_id as u64, rating.rating);
            }

            result.push(MovieUser{id: selected_user.id, name: selected_user.username.clone(), ratings: user_ratings});
        }

        result
    }

    fn get_user_by_id(&self, uid: u64) -> Vec<MovieUser> {
        let query_result = users::table.filter(users::id.eq(uid as i32))
            .limit(1)
            .load::<QueryableUser>(&self.connector)
            .expect("Failed query of user with the uid specified");

        if query_result.is_empty() {
            return Vec::new();
        }

        let selected_user = &query_result[0];

        let query_result = ratings::table.filter(ratings::user_id.eq(selected_user.id))
            .load::<QueryableRating>(&self.connector)
            .expect(&format!("Failed query of ratings of the user {}", selected_user.username));
        
        let mut user_ratings = HashMap::new();
        for rating in &query_result {
            user_ratings.insert(rating.movie_id as u64, rating.rating);
        }

        vec![MovieUser{id: selected_user.id, name: selected_user.username.clone(), ratings: user_ratings}]
    }

    fn get_item_by_name(&self, name: &str) -> Vec<MovieItem> {
        todo!()
    }

    fn get_item_by_id(&self, uid: u64) -> Vec<MovieItem> {
        todo!()
    }

    fn get_all_users(&self) -> Vec<MovieUser> {
        let query_result = users::table
            .load::<QueryableUser>(&self.connector)
            .expect("Failed query of all users");

        let mut result = Vec::new();

        for selected_user in &query_result {
            let query_result = ratings::table.filter(ratings::user_id.eq(selected_user.id))
                .load::<QueryableRating>(&self.connector)
                .expect(&format!("Failed query of ratings of the user {}", selected_user.username));
            
            let mut user_ratings = HashMap::new();
            for rating in &query_result {
                user_ratings.insert(rating.movie_id as u64, rating.rating);
            }

            result.push(MovieUser{id: selected_user.id, name: selected_user.username.clone(), ratings: user_ratings});
        }

        result
    }
}