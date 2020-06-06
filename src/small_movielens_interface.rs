use::std::fmt::Debug;

use std::collections::{HashMap, HashSet};

use db_manager::{DBManager, User, Item};
use simple_movie_db_manager::{movie_db_manager::MovieDBManager, movie_user::MovieUser, movie_item::MovieItem};

fn get_items_by_id_or_name(item_name:Option<String>, item_id:Option<String>) -> Vec<MovieItem> {
    let manager = MovieDBManager::connect_to("postgres://ademir:@localhost/small_movielens");

    let mut item_targets = Vec::new();
    if let Some(item_name) = item_name {
        item_targets = manager.get_item_by_name(&item_name);
    }
    if let Some(item_id) = item_id {
        item_targets = manager.get_item_by_id(item_id.parse().expect("Failed to parse item id"));
    }
    item_targets
}

pub fn get_similarity_matrix() -> (Vec<i32>, Vec<Vec<f64>>){
    let manager = MovieDBManager::connect_to("postgres://ademir:@localhost/small_movielens");

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

pub fn get_similarity_between(
item_order:&Vec<i32>, similarity_matrix:&Vec<Vec<f64>>,
f_item_name:Option<String>, f_item_id:Option<String>,
s_item_name:Option<String>, s_item_id:Option<String>
) -> Option<f64> {
    if (f_item_name == None && f_item_id == None) || (s_item_name == None && s_item_id == None){
        println!("You need to specify an item name or an item id for both items");
        return None;
    }

    let first_items = get_items_by_id_or_name(f_item_name, f_item_id);
    let second_items = get_items_by_id_or_name(s_item_name, s_item_id);

    if first_items.is_empty() || second_items.is_empty() {
        println!("Failed to find the items. FirstResultSize:{}, SecondResultSize:{}", first_items.len(), second_items.len());
        return None;
    }

    let first_item = &first_items[0];
    let second_item = &second_items[0];

    let first_index = item_order.iter().position(|item| *item == first_item.id).expect("First item not found in similarity matrix");
    let second_index = item_order.iter().position(|item| *item == second_item.id).expect("Second item not found in similarity matrix");

    Some(similarity_matrix[first_index][second_index].max(similarity_matrix[second_index][first_index]))
}