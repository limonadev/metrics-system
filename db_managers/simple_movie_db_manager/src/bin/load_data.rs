use std::vec::Vec;

use simple_movie_db_manager::schema::{users, movies, ratings};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use csv;

use simple_movie_db_manager::{movie_user::{NewUser,QueryableUser}, movie_item::{NewMovie,QueryableItem}, movie_rating::{NewRating, QueryableRating}};

fn create_user(conn: &PgConnection, name:&String) -> QueryableUser{
    let new_user =  NewUser{username: name.clone()};

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Failed to insert new_user")
}

fn create_movie(conn: &PgConnection, title:&String) -> QueryableItem{
    let new_movie =  NewMovie{title: title.clone()};

    diesel::insert_into(movies::table)
        .values(&new_movie)
        .get_result(conn)
        .expect("Failed to insert new_movie")
}

fn create_rating(conn: &PgConnection, user_id:i32, movie_id:i32, rating:f64) -> QueryableRating{
    let new_rating =  NewRating{user_id: user_id, movie_id: movie_id, rating:rating};

    diesel::insert_into(ratings::table)
        .values(&new_rating)
        .get_result(conn)
        .expect("Failed to insert new_rating")
}


fn main() {
    let mut content = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("./Movie_Ratings.csv")
        .expect("Couldn't load from csv file");
    
    let mut matrix = Vec::new();

    for r in content.records() {
        if let Ok(record) = r {
            let mut row = Vec::new();
            for r in record.iter() {
                row.push(String::from(r));
            }
            matrix.push(row);
        }
    }

    let connector = PgConnection::establish("postgres://ademir:@localhost/simple_movies").expect("Failed connection to database. Maybe the URL?");

    let mut db_users = Vec::new();

    for col in 1..matrix[0].len() {
        db_users.push(create_user(&connector, &matrix[0][col]));
    }

    let mut db_movies = Vec::new();

    for row in 1..matrix.len() {
        db_movies.push(create_movie(&connector, &matrix[row][0]));
    }


    for i in 0..db_users.len() {
        let current_user = &db_users[i];

        for j in 0..db_movies.len() {
            let current_movie = &db_movies[j];
            if matrix[j+1][i+1] != "" {
                create_rating(&connector, current_user.id, current_movie.id, matrix[j+1][i+1].parse().expect("Failed to parse"));  
            }
        }
    }

}