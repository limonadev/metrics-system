use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

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
    let movie_database_engine = Engine::<i32, i32> {phantom_U: PhantomData, phantom_I: PhantomData};
    let books_database_engine = Engine::<i32, String> {phantom_U: PhantomData, phantom_I: PhantomData};

    let a = movie_database_engine.k_nearest_neighbors(10, HashMap::new());
    let a = books_database_engine.k_nearest_neighbors(10, HashMap::new());

    println!("{}", get_manhattan_distance_by_name(Database::SimpleMovies, String::from("Juan"), String::from("Jose")));
    println!("{}", get_manhattan_distance_by_name(Database::Books, String::from("Juan"), String::from("Jose")));
}
