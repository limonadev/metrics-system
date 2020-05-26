use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

use db_manager::{DBManager, User, Item};
use simple_movie_db_manager::{movie_db_manager::MovieDBManager, movie_user::MovieUser, movie_item::MovieItem};
use book_db_manager::{book_db_manager::BookDBManager, book_user::BookUser, book_item::BookItem};
use small_movielens_db_manager::{small_movielens_db_manager::SmallMovielensDBManager, movie_user::SMovieLensUser, movie_item::SMovieLensItem};

pub struct Engine<U:Hash, I:Hash> {
    phantom_U: PhantomData<U>,
    phantom_I: PhantomData<I>
}

impl<U:Hash,I:Hash> Engine<U,I> {
    fn manhattan_distance_between(&self, first: HashMap<I, f64>, second: HashMap<I, f64>) -> f64 {
        10.0
    }
    fn k_nearest_neighbors(&self, k:i32, ratings:HashMap<U,HashMap<I,f64>>) -> Vec<U>{
        for r in &ratings {
            
        }
        Vec::new()
    }
}

enum Database {
    SimpleMovies,
    Books
}

fn get_manhattan_distance_by_name(database:Database, first:String, second:String) -> f64 {
    match database {
        Database::SimpleMovies => {
            let engine = Engine::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};
            engine.manhattan_distance_between(HashMap::new(), HashMap::new())
        },
        Database::Books => {
            let engine = Engine::<i32,String> {phantom_U: PhantomData, phantom_I: PhantomData};
            engine.manhattan_distance_between(HashMap::new(), HashMap::new())
        }
    }
}

fn main() {
    /*let movie_database_engine = Engine::<i32, i32> {phantom_U: PhantomData, phantom_I: PhantomData};
    let books_database_engine = Engine::<i32, String> {phantom_U: PhantomData, phantom_I: PhantomData};

    let a = movie_database_engine.k_nearest_neighbors(10, HashMap::new());
    let a = books_database_engine.k_nearest_neighbors(10, HashMap::new());

    println!("{}", get_manhattan_distance_by_name(Database::SimpleMovies, String::from("Juan"), String::from("Jose")));
    println!("{}", get_manhattan_distance_by_name(Database::Books, String::from("Juan"), String::from("Jose")));*/

    /*let manager = MovieDBManager::connect_to("postgres://ademir:@localhost/simple_movies");
    println!("{:?}", manager.get_all_ratings());
    let manager = BookDBManager::connect_to("postgres://ademir:@localhost/books");
    println!("{:?}", manager.get_all_ratings());
    let manager = SmallMovielensDBManager::connect_to("postgres://ademir:@localhost/small_movielens");
    println!("{:?}", manager.get_all_ratings());*/
}
