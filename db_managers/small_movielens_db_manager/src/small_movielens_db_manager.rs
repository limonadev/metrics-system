use std::collections::HashMap;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use db_manager::DBManager;

use crate::schema::{users, movies, ratings};
use crate::{movie_user::{SMovieLensUser, QueryableUser}, movie_item::{SMovieLensItem, QueryableItem}, movie_rating::{QueryableRating}};

pub struct SmallMovielensDBManager {
    connector:PgConnection
}

impl DBManager<SMovieLensUser, SMovieLensItem> for SmallMovielensDBManager {
    fn connect_to(url: &str) -> Self {
        let connector = PgConnection::establish(url).expect("Failed connection to database. Maybe the URL?");
        SmallMovielensDBManager{connector: connector}
    }

    fn get_user_by_name(&self, name: &str) -> Vec<SMovieLensUser> {
        vec![]
    }

    fn get_user_by_id(&self, uid: i32) -> Vec<SMovieLensUser> {
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
            user_ratings.insert(rating.movie_id, rating.rating);
        }

        vec![SMovieLensUser{id:selected_user.id, ratings:user_ratings}]
    }

    fn get_item_by_name(&self, name: &str) -> Vec<SMovieLensItem> {
        let query_result = movies::table.filter(movies::title.eq(name))
            .load::<QueryableItem>(&self.connector)
            .expect("Failed query of movie with the given title");

        let mut result = Vec::new();

        for movie in &query_result {
            result.push(
                SMovieLensItem::create(
                    movie.id,
                    movie.title.clone(),
                    movie.genres.clone(),
                )
            );
        }

        result
    }

    fn get_item_by_id(&self, uid: i32) -> Vec<SMovieLensItem> {
        let query_result = movies::table.filter(movies::id.eq(uid))
            .load::<QueryableItem>(&self.connector)
            .expect("Failed query of movie with the given uid");

        if query_result.is_empty() {
            return Vec::new();
        }

        let movie = &query_result[0];

        vec![SMovieLensItem::create(movie.id, movie.title.clone(), movie.genres.clone())]
    }

    fn get_all_users(&self) -> Vec<SMovieLensUser> {
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
                user_ratings.insert(rating.movie_id, rating.rating);
            }

            result.push(SMovieLensUser{id: selected_user.id, ratings: user_ratings});
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
        let user_chunk = users::table
            .limit(limit)
            .offset(offset)
            .load::<QueryableUser>(&self.connector)
            .expect("Failed to fetch chunk of users");

        let query_result = QueryableRating::belonging_to(&user_chunk).load::<QueryableRating>(&self.connector).expect("Failed to fetch chunk of ratings");

        let mut users_with_ratings = HashMap::new();
        for rating in query_result {
            if !users_with_ratings.contains_key(&rating.user_id) {
                users_with_ratings.insert(rating.user_id, HashMap::new());
            }

            users_with_ratings.get_mut(&rating.user_id).unwrap().insert(rating.movie_id, rating.rating);
        }

        users_with_ratings
    }
    
    

}