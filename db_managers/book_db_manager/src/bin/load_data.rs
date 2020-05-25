use book_db_manager::schema::{users, books, ratings};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use csv;
use indicatif::ProgressIterator;

use book_db_manager::{book_user::{NewUser,QueryableUser}, book_item::{NewBook,QueryableItem}, book_rating::{NewRating, QueryableRating}};
use db_manager::DBManager;
use book_db_manager::book_db_manager::BookDBManager;

fn create_user(id:i32, city:&String, age:Option<i32>) -> NewUser{
    let new_user =  NewUser{id: id, city:city.clone(), age:age};
    new_user
}

fn create_book(id:&String, title:&String, author:&String, pub_year:&String, publisher:&String) -> NewBook{
    let new_book =  NewBook{id: id.clone(), title: title.clone(), author: author.clone(), pub_year: pub_year.clone(), publisher: publisher.clone()};
    new_book
}

fn create_rating(user_id:i32, book_id:&String, rating:f64) -> NewRating{
    let new_rating =  NewRating{user_id: user_id, book_id: book_id.clone(), rating:rating};
    new_rating
    
}


fn main() {
    let connector = PgConnection::establish("postgres://ademir:@localhost/books").expect("Failed connection to database. Maybe the URL?");

    let mut users_file = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_path("./BOOKS-DB/BX-Users.csv")
        .expect("Couldn't load from users csv file");

    let mut users_to_insert = Vec::new();

    for r in users_file.records().progress() {
        if let Ok(record) = r {
            let id = record[0].parse().expect("Failed to parse user id");
            let city = &record[1].to_string();
            let age:Option<i32> = if &record[2] == String::from("\\N") {
                None
            }else {
                Some(record[2].parse().expect("Failed to parse age"))
            };
            users_to_insert.push(create_user(id, city, age));
        }
    }

    for i_users in users_to_insert.chunks(10000).progress() {
        diesel::insert_into(users::table).values(i_users).execute(&connector).expect("Failed insertion of users chunk");
    }

    let mut books_file = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_path("./BOOKS-DB/BX-Books.csv")
        .expect("Couldn't load from books csv file");

    let mut books_to_insert = Vec::new();

    for r in books_file.records().progress() {
        if let Ok(record) = r {
            let id = &record[0].to_string();
            let title = &record[1].to_string();
            let author = &record[2].to_string();
            let pub_year = &record[3].to_string();
            let publisher = &record[4].to_string();
            books_to_insert.push(create_book(id, title, author, pub_year, publisher));
        }
    }

    for i_books in books_to_insert.chunks(10000).progress() {
        diesel::insert_into(books::table).values(i_books).execute(&connector).expect("Failed insertion of books chunk");
    }

    let mut ratings_file = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_path("./BOOKS-DB/BX-Book-Ratings.csv")
        .expect("Couldn't load from ratings csv file");

    let mut ratings_to_insert = Vec::new();
    let manager = BookDBManager::connect_to("postgres://ademir:@localhost/books");

    for r in ratings_file.records().progress() {
        if let Ok(record) = r {
            let user_id = record[0].parse().expect("Failed to parse the user id of the rating");
            let book_id = &record[1].to_string();
            let rating = record[2].parse().expect("Failed to parse the rating");
            if manager.get_user_by_id(user_id).is_empty() {
                continue;
            }
            if manager.get_item_by_id(book_id.clone()).is_empty() {
                continue;
            }
            ratings_to_insert.push(create_rating(user_id, book_id, rating));
        }
    }

    for i_rating in ratings_to_insert.chunks(10000).progress() {
        diesel::insert_into(ratings::table).values(i_rating).execute(&connector).expect("Failed insertion of books chunk");
    }

}