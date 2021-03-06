use std::collections::HashMap;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use db_manager::DBManager;

use crate::schema::{users, movies, ratings};
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
                user_ratings.insert(rating.movie_id, rating.rating);
            }

            result.push(MovieUser{id: selected_user.id, name: selected_user.username.clone(), ratings: user_ratings});
        }

        result
    }

    fn get_user_by_id(&self, uid: i32) -> Vec<MovieUser> {
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
            user_ratings.insert(rating.movie_id, rating.rating);
        }

        vec![MovieUser{id: selected_user.id, name: selected_user.username.clone(), ratings: user_ratings}]
    }

    fn get_item_by_name(&self, name: &str) -> Vec<MovieItem> {
        let query_result = movies::table.filter(movies::title.eq(name))
            .load::<QueryableItem>(&self.connector)
            .expect("Failed query of movie with the given title");

        let mut result = Vec::new();

        for movie in &query_result {
            result.push(MovieItem{id: movie.id, name: movie.title.clone()});
        }

        result
    }

    fn get_item_by_id(&self, uid: i32) -> Vec<MovieItem> {
        let query_result = movies::table.filter(movies::id.eq(uid as i32))
            .load::<QueryableItem>(&self.connector)
            .expect("Failed query of movie with the given uid");

        if query_result.is_empty() {
            return Vec::new();
        }

        vec![MovieItem{id: query_result[0].id, name: query_result[0].title.clone()}]
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
                user_ratings.insert(rating.movie_id, rating.rating);
            }

            result.push(MovieUser{id: selected_user.id, name: selected_user.username.clone(), ratings: user_ratings});
        }

        result
    }

    fn get_all_ratings(&self) -> HashMap<i32, HashMap<i32, f64>> {
        let query_result = ratings::table
            .load::<QueryableRating>(&self.connector)
            .expect("Failed to fetch all ratings");

        let mut result = HashMap::new();
        for rating in &query_result {
            if !result.contains_key(&rating.user_id) {
                result.insert(rating.user_id, HashMap::new());
            }
            let user_ratings = result.get_mut(&rating.user_id).unwrap();
            user_ratings.insert(rating.movie_id, rating.rating);
        }
        result
    }
    fn get_users_chunk(&self, offset: i64, limit: i64) -> Vec<i32> {
        let user_chunk = users::table
            .select(users::id)
            .limit(limit)
            .offset(offset)
            .load::<i32>(&self.connector)
            .expect("Failed to fetch chunk of users");
        
        user_chunk
    }
    fn get_user_ratings(&self, uid: i32) -> HashMap<i32, f64> {
        let user:QueryableUser = users::table.find(uid).get_result(&self.connector).unwrap();
        let query_result = QueryableRating::belonging_to(&user).load::<QueryableRating>(&self.connector).unwrap();

        let mut ratings_by_item = HashMap::new();
        for rating in &query_result {
            ratings_by_item.insert(rating.movie_id, rating.rating);
        }

        ratings_by_item
    }
    fn get_users_with_ratings_chunk(&self, offset: i64, limit: i64) -> HashMap<i32, HashMap<i32, f64>> {
        /*let mut users_with_ratings = HashMap::new();

        for uid in users_id {
            let user:QueryableUser = users::table.find(uid).get_result(&self.connector).unwrap();
            let query_result = QueryableRating::belonging_to(&user).load::<QueryableRating>(&self.connector).unwrap();
            let mut ratings_by_item = HashMap::new();
            for rating in &query_result {
                ratings_by_item.insert(rating.movie_id, rating.rating);
            }

            users_with_ratings.insert(*uid, ratings_by_item);
        }
        
        users_with_ratings*/
        HashMap::new()
    }

    
    
}