use std::collections::HashSet;

use small_movielens_db_manager::schema::{users, movies, ratings};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use csv;
use indicatif::ProgressIterator;

use small_movielens_db_manager::{movie_user::{NewUser}, movie_item::{NewMovie}, movie_rating::{NewRating}};
use db_manager::DBManager;
use small_movielens_db_manager::small_movielens_db_manager::SmallMovielensDBManager;

/*fn create_book(id:&String, title:&String, author:&String, pub_year:&String, publisher:&String) -> NewBook{
    let new_book =  NewBook{id: id.clone(), title: title.clone(), author: author.clone(), pub_year: pub_year.clone(), publisher: publisher.clone()};
    new_book
}

fn create_rating(user_id:i32, book_id:&String, rating:f64) -> NewRating{
    let new_rating =  NewRating{user_id: user_id, book_id: book_id.clone(), rating:rating};
    new_rating
    
}*/


fn main() {
    let connector = PgConnection::establish("postgres://ademir:@localhost/small_movielens").expect("Failed connection to database. Maybe the URL?");

    let mut users_file = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_path("./ml-latest-small/ratings.csv")
        .expect("Couldn't load from users csv file");

    let mut users_ids = HashSet::new();
    let mut users_to_insert = Vec::new();

    for r in users_file.records().progress() {
        if let Ok(record) = r {
            let id:i32 = record[0].parse().expect("Failed to parse user id");
            if !users_ids.contains(&id) {
                users_ids.insert(id);
                users_to_insert.push(NewUser{id: id});
            }
        }
    }

    for i_users in users_to_insert.chunks(10000).progress() {
        diesel::insert_into(users::table).values(i_users).execute(&connector).expect("Failed insertion of users chunk");
    }

    let mut movies_file = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_path("./ml-latest-small/movies.csv")
        .expect("Couldn't load from movies csv file");

    let mut movies_to_insert = Vec::new();

    for r in movies_file.records().progress() {
        if let Ok(record) = r {
            let id = record[0].parse().expect("Failed to parse the movie id");
            let title = &record[1].to_string();
            let genres = &record[2].to_string();
            movies_to_insert.push(NewMovie{id: id, title: title.clone(), genres: genres.clone()});
        }
    }

    for i_movies in movies_to_insert.chunks(10000).progress() {
        diesel::insert_into(movies::table).values(i_movies).execute(&connector).expect("Failed insertion of movies chunk");
    }

    let mut ratings_file = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_path("./ml-latest-small/ratings.csv")
        .expect("Couldn't load from ratings csv file");

    let mut ratings_to_insert = Vec::new();
    let manager = SmallMovielensDBManager::connect_to("postgres://ademir:@localhost/small_movielens");

    for r in ratings_file.records().progress() {
        if let Ok(record) = r {
            let user_id = record[0].parse().expect("Failed to parse the user id of the rating");
            let movie_id = record[1].parse().expect("Failed to parse the movie id of the rating");
            let rating = record[2].parse().expect("Failed to parse the rating");
            if manager.get_user_by_id(user_id).is_empty() {
                continue;
            }
            if manager.get_item_by_id(movie_id).is_empty() {
                continue;
            }
            ratings_to_insert.push(NewRating{user_id: user_id, movie_id: movie_id, rating: rating});
        }
    }

    for i_rating in ratings_to_insert.chunks(10000).progress() {
        diesel::insert_into(ratings::table).values(i_rating).execute(&connector).expect("Failed insertion of ratings chunk");
    }

}