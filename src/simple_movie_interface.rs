use::std::fmt::Debug;

use std::collections::{HashMap, HashSet};
use std::collections::BinaryHeap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::cmp::{Reverse, min};

use db_manager::{DBManager, User, Item};
use simple_movie_db_manager::{movie_db_manager::MovieDBManager, movie_user::MovieUser, movie_item::MovieItem};

pub fn get_similarity_matrix() -> (Vec<i32>, Vec<Vec<f64>>){
    let manager = MovieDBManager::connect_to("postgres://ademir:@localhost/simple_movies");

    let mut all_ratings = manager.get_all_ratings();
    let mut averages = HashMap::new();

    let mut users_deviations = HashMap::new();
    let mut users_per_movie = HashMap::new();
    let mut movies_order = Vec::new();

    for (user_id,user_ratings) in &all_ratings {
        let user_average_rating: f64 = user_ratings.values().sum();
        let user_average_rating = user_average_rating/user_ratings.len() as f64;

        averages.insert(*user_id, user_average_rating);

        users_deviations.insert(*user_id, HashMap::new());

        for (movie_id, rating) in user_ratings {
            if !users_per_movie.contains_key(movie_id) {
                users_per_movie.insert(*movie_id, HashSet::new());
                movies_order.push(*movie_id);
            }

            let deviation = rating - user_average_rating;
            users_per_movie.get_mut(movie_id).unwrap().insert(*user_id);
            users_deviations.get_mut(user_id).unwrap().insert(*movie_id, deviation);
        }
    }
    all_ratings.clear();

    let mut similarity_matrix = Vec::new();

    let row_size = movies_order.len();
    for (i, movie_id) in (&movies_order).iter().enumerate() {
        let mut row = vec![-f64::INFINITY; i];
        
        for j in i..row_size {
            let other_movie_id = &movies_order[j];
            let common_users = users_per_movie[movie_id].intersection(&users_per_movie[other_movie_id]);

            let mut numerator = 0.0;
            let mut first_square = 0.0;
            let mut second_square = 0.0;
            for user_id in common_users {
                let first_deviation = users_deviations[user_id][movie_id];
                let second_deviation = users_deviations[user_id][other_movie_id];

                numerator += first_deviation*second_deviation;
                first_square += first_deviation.powi(2);
                second_square += second_deviation.powi(2);
            }

            let denominator = first_square.sqrt() * second_square.sqrt();
            row.push(numerator/denominator);
        }

        similarity_matrix.push(row);
    }

    println!("{}", movies_order.len());
    //println!("{:?}", movies_order);
    //println!("{:#?}", similarity_matrix);

    (movies_order, similarity_matrix)
}