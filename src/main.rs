use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

use db_manager::{DBManager, User, Item};
use simple_movie_db_manager::{movie_db_manager::MovieDBManager, movie_user::MovieUser, movie_item::MovieItem};
use book_db_manager::{book_db_manager::BookDBManager, book_user::BookUser, book_item::BookItem};
use small_movielens_db_manager::{small_movielens_db_manager::SmallMovielensDBManager, movie_user::SMovieLensUser, movie_item::SMovieLensItem};

pub struct Engine<U, I> {
    phantom_U: PhantomData<U>,
    phantom_I: PhantomData<I>
}

impl<U:Hash+Eq,I:Hash+Eq> Engine<U,I> {
    fn manhattan_distance_between(&self, first: &HashMap<I, f64>, second: &HashMap<I, f64>) -> f64 {
        Self::minkowski_distance_between(self, first, second, 1)
    }

    fn euclidean_distance_between(&self, first: &HashMap<I, f64>, second: &HashMap<I, f64>) -> f64 {
        Self::minkowski_distance_between(self, first, second, 2)
    }

    fn minkowski_distance_between(&self, first: &HashMap<I, f64>, second: &HashMap<I, f64>, grade: i32) -> f64 {
        let mut distance = 0.0;

        for (item_id, first_ranking) in first {
            if let Some(second_ranking) = second.get(item_id) {
                let diff = (first_ranking-second_ranking).abs().powi(grade);
                distance += diff;
            }
        }
        distance
    }

    fn cosine_similarity_between(&self, first: &HashMap<I, f64>, second: &HashMap<I, f64>) -> f64 {
        let mut first_len = 0.0;
        let mut second_len = 0.0;
        let mut pointwise_sum = 0.0;
        
        for (item_id, first_ranking) in first {
            if let Some(second_ranking) = second.get(item_id) {
                pointwise_sum += first_ranking*second_ranking;
                first_len += first_ranking.powi(2);
                second_len += second_ranking.powi(2);
            }
        }

        first_len = first_len.sqrt();
        second_len = second_len.sqrt();
        pointwise_sum/(first_len*second_len)
    }

    fn jaccard_distance_between(&self, first: &HashMap<I, f64>, second: &HashMap<I, f64>) -> f64 {
        let mut intersection = 0;
        for item_id in first.keys() {
            if second.contains_key(item_id){
                intersection += 1;
            }
        }
        let union = (first.keys().len() - intersection) + (second.keys().len() - intersection) + intersection;
        intersection as f64/union as f64
    }

    fn jaccard_index_between(&self, first: &HashMap<I, f64>, second: &HashMap<I, f64>) -> f64 {
        1.0 - Self::jaccard_distance_between(self, first, second)
    }

    fn k_nearest_neighbors(&self, k:i32, ratings:HashMap<U,HashMap<I,f64>>) -> Vec<U>{
        for r in &ratings {
            
        }
        Vec::new()
    }
}

enum Database {
    SimpleMovies{url:String},
    Books{url:String},
    SmallMovieLens{url:String}
}

fn get_manhattan_distance_by_name(database:&Database, first:String, second:String) {
    match database {
        Database::SimpleMovies{url} => {
            let manager = MovieDBManager::connect_to(&url);
            let engine = Engine::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_name(&first);
            let users_named_second = manager.get_user_by_name(&second);

            if users_named_first.is_empty() {
                println!("No user with name {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with name {} found!", second);
                return;
            }

            for first_user in &users_named_first {
                let current_first_ratings = first_user.ratings();
                for second_user in &users_named_second {
                    let current_second_ratings = second_user.ratings();
                    let distance = engine.manhattan_distance_between(&current_first_ratings, &current_second_ratings);
                    println!("SimpleMovies Manhattan Distance between {} with id {} and {} with id {} is {}", first, first_user.id, second, second_user.id, distance);
                }
            }
        },
        Database::Books{url} => {
            println!("Books Database has not user names!");
        }
        Database::SmallMovieLens{url} => {
            println!("Small MovieLens Database has not user names!");
        }
    }
}

fn get_manhattan_distance_by_id(database:&Database, first:String, second:String) {
    match database {
        Database::SimpleMovies{url} => {
            let manager = MovieDBManager::connect_to(&url);
            let engine = Engine::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_id(first.parse().expect("Failed to parse first id in Manhattan"));
            let users_named_second = manager.get_user_by_id(second.parse().expect("Failed to parse second id in Manhattan"));

            if users_named_first.is_empty() {
                println!("No user with id {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with id {} found!", second);
                return;
            }

            let first_user = &users_named_first[0];
            let second_user = &users_named_second[0];

            let distance = engine.manhattan_distance_between(&first_user.ratings(), &second_user.ratings());

            println!("SimpleMovies Manhattan Distance between id {} and id {} is {}", first, second, distance);
        },
        Database::Books{url} => {
            let manager = BookDBManager::connect_to(&url);
            let engine = Engine::<i32,String> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_id(first.parse().expect("Failed to parse first id in Manhattan"));
            let users_named_second = manager.get_user_by_id(second.parse().expect("Failed to parse second id in Manhattan"));

            if users_named_first.is_empty() {
                println!("No user with id {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with id {} found!", second);
                return;
            }

            let first_user = &users_named_first[0];
            let second_user = &users_named_second[0];

            let distance = engine.manhattan_distance_between(&first_user.ratings(), &second_user.ratings());

            println!("Books Manhattan Distance between {} and {} is {}", first_user.id, second_user.id, distance);
        }
        Database::SmallMovieLens{url} => {
            let manager = SmallMovielensDBManager::connect_to(&url);
            let engine = Engine::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_id(first.parse().expect("Failed to parse first id in Manhattan"));
            let users_named_second = manager.get_user_by_id(second.parse().expect("Failed to parse second id in Manhattan"));

            if users_named_first.is_empty() {
                println!("No user with id {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with id {} found!", second);
                return;
            }

            let first_user = &users_named_first[0];
            let second_user = &users_named_second[0];

            let distance = engine.manhattan_distance_between(&first_user.ratings(), &second_user.ratings());

            println!("SmallMovieLens Manhattan Distance between {} and {} is {}", first_user.id, second_user.id, distance);
        }
    }
}

fn get_euclidean_distance_by_name(database:&Database, first:String, second:String) {
    match database {
        Database::SimpleMovies{url} => {
            let manager = MovieDBManager::connect_to(&url);
            let engine = Engine::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_name(&first);
            let users_named_second = manager.get_user_by_name(&second);

            if users_named_first.is_empty() {
                println!("No user with name {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with name {} found!", second);
                return;
            }

            for first_user in &users_named_first {
                let current_first_ratings = first_user.ratings();
                for second_user in &users_named_second {
                    let current_second_ratings = second_user.ratings();
                    let distance = engine.euclidean_distance_between(&current_first_ratings, &current_second_ratings);
                    println!("SimpleMovies Euclidean Distance between {} with id {} and {} with id {} is {}", first, first_user.id, second, second_user.id, distance);
                }
            }
        },
        Database::Books{url} => {
            println!("Books Database has not user names!");
        }
        Database::SmallMovieLens{url} => {
            println!("Small MovieLens Database has not user names!");
        }
    }
}

fn get_euclidean_distance_by_id(database:&Database, first:String, second:String) {
    match database {
        Database::SimpleMovies{url} => {
            let manager = MovieDBManager::connect_to(&url);
            let engine = Engine::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_id(first.parse().expect("Failed to parse first id in Euclidean"));
            let users_named_second = manager.get_user_by_id(second.parse().expect("Failed to parse second id in Euclidean"));

            if users_named_first.is_empty() {
                println!("No user with id {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with id {} found!", second);
                return;
            }

            let first_user = &users_named_first[0];
            let second_user = &users_named_second[0];

            let distance = engine.euclidean_distance_between(&first_user.ratings(), &second_user.ratings());

            println!("SimpleMovies Euclidean Distance between id {} and id {} is {}", first, second, distance);
        },
        Database::Books{url} => {
            let manager = BookDBManager::connect_to(&url);
            let engine = Engine::<i32,String> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_id(first.parse().expect("Failed to parse first id in Euclidean"));
            let users_named_second = manager.get_user_by_id(second.parse().expect("Failed to parse second id in Euclidean"));

            if users_named_first.is_empty() {
                println!("No user with id {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with id {} found!", second);
                return;
            }

            let first_user = &users_named_first[0];
            let second_user = &users_named_second[0];

            let distance = engine.euclidean_distance_between(&first_user.ratings(), &second_user.ratings());

            println!("Books Euclidean Distance between {} and {} is {}", first_user.id, second_user.id, distance);
        }
        Database::SmallMovieLens{url} => {
            let manager = SmallMovielensDBManager::connect_to(&url);
            let engine = Engine::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_id(first.parse().expect("Failed to parse first id in Euclidean"));
            let users_named_second = manager.get_user_by_id(second.parse().expect("Failed to parse second id in Euclidean"));

            if users_named_first.is_empty() {
                println!("No user with id {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with id {} found!", second);
                return;
            }

            let first_user = &users_named_first[0];
            let second_user = &users_named_second[0];

            let distance = engine.euclidean_distance_between(&first_user.ratings(), &second_user.ratings());

            println!("SmallMovieLens Euclidean Distance between {} and {} is {}", first_user.id, second_user.id, distance);
        }
    }
}

fn get_minkowski_distance_by_name(database:&Database, first:String, second:String, grade:i32) {
    match database {
        Database::SimpleMovies{url} => {
            let manager = MovieDBManager::connect_to(&url);
            let engine = Engine::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_name(&first);
            let users_named_second = manager.get_user_by_name(&second);

            if users_named_first.is_empty() {
                println!("No user with name {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with name {} found!", second);
                return;
            }

            for first_user in &users_named_first {
                let current_first_ratings = first_user.ratings();
                for second_user in &users_named_second {
                    let current_second_ratings = second_user.ratings();
                    let distance = engine.minkowski_distance_between(&current_first_ratings, &current_second_ratings, grade);
                    println!("SimpleMovies Minkowski Distance with grade {} between {} with id {} and {} with id {} is {}", grade, first, first_user.id, second, second_user.id, distance);
                }
            }
        },
        Database::Books{url} => {
            println!("Books Database has not user names!");
        }
        Database::SmallMovieLens{url} => {
            println!("Small MovieLens Database has not user names!");
        }
    }
}

fn get_minkowski_distance_by_id(database:&Database, first:String, second:String, grade:i32) {
    match database {
        Database::SimpleMovies{url} => {
            let manager = MovieDBManager::connect_to(&url);
            let engine = Engine::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_id(first.parse().expect("Failed to parse first id in Minkowski"));
            let users_named_second = manager.get_user_by_id(second.parse().expect("Failed to parse second id in Minkowski"));

            if users_named_first.is_empty() {
                println!("No user with id {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with id {} found!", second);
                return;
            }

            let first_user = &users_named_first[0];
            let second_user = &users_named_second[0];

            let distance = engine.minkowski_distance_between(&first_user.ratings(), &second_user.ratings(), grade);

            println!("SimpleMovies Minkowski Distance with grade {} between id {} and id {} is {}", grade, first, second, distance);
        },
        Database::Books{url} => {
            let manager = BookDBManager::connect_to(&url);
            let engine = Engine::<i32,String> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_id(first.parse().expect("Failed to parse first id in Minkowski"));
            let users_named_second = manager.get_user_by_id(second.parse().expect("Failed to parse second id in Minkowski"));

            if users_named_first.is_empty() {
                println!("No user with id {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with id {} found!", second);
                return;
            }

            let first_user = &users_named_first[0];
            let second_user = &users_named_second[0];

            let distance = engine.minkowski_distance_between(&first_user.ratings(), &second_user.ratings(), grade);

            println!("Books Minkowski Distance with grade {} between id {} and id {} is {}", grade, first_user.id, second_user.id, distance);
        }
        Database::SmallMovieLens{url} => {
            let manager = SmallMovielensDBManager::connect_to(&url);
            let engine = Engine::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_id(first.parse().expect("Failed to parse first id in Minkowski"));
            let users_named_second = manager.get_user_by_id(second.parse().expect("Failed to parse second id in Minkowski"));

            if users_named_first.is_empty() {
                println!("No user with id {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with id {} found!", second);
                return;
            }

            let first_user = &users_named_first[0];
            let second_user = &users_named_second[0];

            let distance = engine.minkowski_distance_between(&first_user.ratings(), &second_user.ratings(), grade);

            println!("SmallMovieLens Minkowski Distance with grade {} between id {} and id {} is {}", grade, first_user.id, second_user.id, distance);
        }
    }
}

fn get_cosine_similarity_by_name(database:&Database, first:String, second:String) {
    match database {
        Database::SimpleMovies{url} => {
            let manager = MovieDBManager::connect_to(&url);
            let engine = Engine::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_name(&first);
            let users_named_second = manager.get_user_by_name(&second);

            if users_named_first.is_empty() {
                println!("No user with name {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with name {} found!", second);
                return;
            }

            for first_user in &users_named_first {
                let current_first_ratings = first_user.ratings();
                for second_user in &users_named_second {
                    let current_second_ratings = second_user.ratings();
                    let similarity = engine.cosine_similarity_between(&current_first_ratings, &current_second_ratings);
                    println!("SimpleMovies Cosine Similarity between {} and {} is {}", first_user.name, second_user.name, similarity);
                }
            }
        },
        Database::Books{url} => {
            println!("Books Database has not user names!");
        }
        Database::SmallMovieLens{url} => {
            println!("Small MovieLens Database has not user names!");
        }
    }
}

fn get_cosine_similarity_by_id(database:&Database, first:String, second:String) {
    match database {
        Database::SimpleMovies{url} => {
            let manager = MovieDBManager::connect_to(&url);
            let engine = Engine::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_id(first.parse().expect("Failed to parse first id in Cosine Similarity"));
            let users_named_second = manager.get_user_by_id(second.parse().expect("Failed to parse second id in COsine Similarity"));

            if users_named_first.is_empty() {
                println!("No user with id {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with id {} found!", second);
                return;
            }

            let first_user = &users_named_first[0];
            let second_user = &users_named_second[0];

            let similarity = engine.cosine_similarity_between(&first_user.ratings(), &second_user.ratings());

            println!("SimpleMovies Cosine Similarity between id {} and id {} is {}", first_user.id, second_user.id, similarity);
        },
        Database::Books{url} => {
            let manager = BookDBManager::connect_to(&url);
            let engine = Engine::<i32,String> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_id(first.parse().expect("Failed to parse first id in Cosine Similarity"));
            let users_named_second = manager.get_user_by_id(second.parse().expect("Failed to parse second id in Cosine Similarity"));

            if users_named_first.is_empty() {
                println!("No user with id {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with id {} found!", second);
                return;
            }

            let first_user = &users_named_first[0];
            let second_user = &users_named_second[0];

            let similarity = engine.cosine_similarity_between(&first_user.ratings(), &second_user.ratings());

            println!("Books Cosine Similarity between id {} and id {} is {}", first_user.id, second_user.id, similarity);
        }
        Database::SmallMovieLens{url} => {
            let manager = SmallMovielensDBManager::connect_to(&url);
            let engine = Engine::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_id(first.parse().expect("Failed to parse first id in Cosine Similarity"));
            let users_named_second = manager.get_user_by_id(second.parse().expect("Failed to parse second id in Cosine Similarity"));

            if users_named_first.is_empty() {
                println!("No user with id {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with id {} found!", second);
                return;
            }

            let first_user = &users_named_first[0];
            let second_user = &users_named_second[0];

            let similarity = engine.cosine_similarity_between(&first_user.ratings(), &second_user.ratings());

            println!("SmallMovieLens Cosine Similarity between id {} and id {} is {}", first_user.id, second_user.id, similarity);
        }
    }
}

fn get_jaccard_distance_by_name(database:&Database, first:String, second:String) {
    match database {
        Database::SimpleMovies{url} => {
            let manager = MovieDBManager::connect_to(&url);
            let engine = Engine::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_name(&first);
            let users_named_second = manager.get_user_by_name(&second);

            if users_named_first.is_empty() {
                println!("No user with name {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with name {} found!", second);
                return;
            }

            for first_user in &users_named_first {
                let current_first_ratings = first_user.ratings();
                for second_user in &users_named_second {
                    let current_second_ratings = second_user.ratings();
                    let distance = engine.jaccard_distance_between(&current_first_ratings, &current_second_ratings);
                    println!("SimpleMovies Jaccard Distance between {} and {} is {}", first_user.name, second_user.name, distance);
                }
            }
        },
        Database::Books{url} => {
            println!("Books Database has not user names!");
        }
        Database::SmallMovieLens{url} => {
            println!("Small MovieLens Database has not user names!");
        }
    }
}

fn get_jaccard_distance_by_id(database:&Database, first:String, second:String) {
    match database {
        Database::SimpleMovies{url} => {
            let manager = MovieDBManager::connect_to(&url);
            let engine = Engine::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_id(first.parse().expect("Failed to parse first id in Jaccard Distance"));
            let users_named_second = manager.get_user_by_id(second.parse().expect("Failed to parse second id in Jaccard Distance"));

            if users_named_first.is_empty() {
                println!("No user with id {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with id {} found!", second);
                return;
            }

            let first_user = &users_named_first[0];
            let second_user = &users_named_second[0];

            let distance = engine.jaccard_distance_between(&first_user.ratings(), &second_user.ratings());

            println!("SimpleMovies Jaccard Distance between id {} and id {} is {}", first_user.id, second_user.id, distance);
        },
        Database::Books{url} => {
            let manager = BookDBManager::connect_to(&url);
            let engine = Engine::<i32,String> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_id(first.parse().expect("Failed to parse first id in Jaccard Distance"));
            let users_named_second = manager.get_user_by_id(second.parse().expect("Failed to parse second id in Jaccard Distance"));

            if users_named_first.is_empty() {
                println!("No user with id {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with id {} found!", second);
                return;
            }

            let first_user = &users_named_first[0];
            let second_user = &users_named_second[0];

            let distance = engine.jaccard_distance_between(&first_user.ratings(), &second_user.ratings());

            println!("Books Jaccard Distance between id {} and id {} is {}", first_user.id, second_user.id, distance);
        }
        Database::SmallMovieLens{url} => {
            let manager = SmallMovielensDBManager::connect_to(&url);
            let engine = Engine::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_id(first.parse().expect("Failed to parse first id in Jaccard Distance"));
            let users_named_second = manager.get_user_by_id(second.parse().expect("Failed to parse second id in Jaccard Distance"));

            if users_named_first.is_empty() {
                println!("No user with id {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with id {} found!", second);
                return;
            }

            let first_user = &users_named_first[0];
            let second_user = &users_named_second[0];

            let distance = engine.jaccard_distance_between(&first_user.ratings(), &second_user.ratings());

            println!("SmallMovieLens Jaccard Distance between id {} and id {} is {}", first_user.id, second_user.id, distance);
        }
    }
}

fn get_jaccard_index_by_name(database:&Database, first:String, second:String) {
    match database {
        Database::SimpleMovies{url} => {
            let manager = MovieDBManager::connect_to(&url);
            let engine = Engine::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_name(&first);
            let users_named_second = manager.get_user_by_name(&second);

            if users_named_first.is_empty() {
                println!("No user with name {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with name {} found!", second);
                return;
            }

            for first_user in &users_named_first {
                let current_first_ratings = first_user.ratings();
                for second_user in &users_named_second {
                    let current_second_ratings = second_user.ratings();
                    let index = engine.jaccard_index_between(&current_first_ratings, &current_second_ratings);
                    println!("SimpleMovies Jaccard Index between {} and {} is {}", first_user.name, second_user.name, index);
                }
            }
        },
        Database::Books{url} => {
            println!("Books Database has not user names!");
        }
        Database::SmallMovieLens{url} => {
            println!("Small MovieLens Database has not user names!");
        }
    }
}

fn get_jaccard_index_by_id(database:&Database, first:String, second:String) {
    match database {
        Database::SimpleMovies{url} => {
            let manager = MovieDBManager::connect_to(&url);
            let engine = Engine::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_id(first.parse().expect("Failed to parse first id in Jaccard Index"));
            let users_named_second = manager.get_user_by_id(second.parse().expect("Failed to parse second id in Jaccard Index"));

            if users_named_first.is_empty() {
                println!("No user with id {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with id {} found!", second);
                return;
            }

            let first_user = &users_named_first[0];
            let second_user = &users_named_second[0];

            let index = engine.jaccard_index_between(&first_user.ratings(), &second_user.ratings());

            println!("SimpleMovies Jaccard Index between id {} and id {} is {}", first_user.id, second_user.id, index);
        },
        Database::Books{url} => {
            let manager = BookDBManager::connect_to(&url);
            let engine = Engine::<i32,String> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_id(first.parse().expect("Failed to parse first id in Jaccard Index"));
            let users_named_second = manager.get_user_by_id(second.parse().expect("Failed to parse second id in Jaccard Index"));

            if users_named_first.is_empty() {
                println!("No user with id {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with id {} found!", second);
                return;
            }

            let first_user = &users_named_first[0];
            let second_user = &users_named_second[0];

            let index = engine.jaccard_index_between(&first_user.ratings(), &second_user.ratings());

            println!("Books Jaccard Index between id {} and id {} is {}", first_user.id, second_user.id, index);
        }
        Database::SmallMovieLens{url} => {
            let manager = SmallMovielensDBManager::connect_to(&url);
            let engine = Engine::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};

            let users_named_first = manager.get_user_by_id(first.parse().expect("Failed to parse first id in Jaccard Index"));
            let users_named_second = manager.get_user_by_id(second.parse().expect("Failed to parse second id in Jaccard Index"));

            if users_named_first.is_empty() {
                println!("No user with id {} found!", first);
                return;
            }
            if users_named_second.is_empty() {
                println!("No user with id {} found!", second);
                return;
            }

            let first_user = &users_named_first[0];
            let second_user = &users_named_second[0];

            let index = engine.jaccard_index_between(&first_user.ratings(), &second_user.ratings());

            println!("SmallMovieLens Jaccard Index between id {} and id {} is {}", first_user.id, second_user.id, index);
        }
    }
}


fn main() {

    let simple_movies_database = Database::SimpleMovies{url: String::from("postgres://ademir:@localhost/simple_movies")};
    let books_database = Database::Books{url: String::from("postgres://ademir:@localhost/books")};
    let small_movielens_database = Database::SmallMovieLens{url: String::from("postgres://ademir:@localhost/small_movielens")};
    
    //get_manhattan_distance_by_name(simple_movies_database, String::new(), String::new());

    get_manhattan_distance_by_name(&simple_movies_database, String::from("Patrick C"), String::from("Heather"));
    get_manhattan_distance_by_id(&simple_movies_database, String::from("1"), String::from("2"));

    get_minkowski_distance_by_name(&simple_movies_database, String::from("Patrick C"), String::from("Heather"), 1);
    get_minkowski_distance_by_id(&simple_movies_database, String::from("1"), String::from("2"), 1);

    get_euclidean_distance_by_name(&simple_movies_database, String::from("Patrick C"), String::from("Heather"));
    get_euclidean_distance_by_id(&simple_movies_database, String::from("1"), String::from("2"));

    get_minkowski_distance_by_name(&simple_movies_database, String::from("Patrick C"), String::from("Heather"), 2);
    get_minkowski_distance_by_id(&simple_movies_database, String::from("1"), String::from("2"), 2);
    //get_minkowski_distance_by_id(&books_database, String::from("26182"), String::from("269352"), 2);

    get_cosine_similarity_by_name(&simple_movies_database, String::from("Patrick C"), String::from("Heather"));
    get_cosine_similarity_by_id(&simple_movies_database, String::from("1"), String::from("2"));

    get_jaccard_distance_by_name(&simple_movies_database, String::from("Patrick C"), String::from("Heather"));
    get_jaccard_distance_by_id(&simple_movies_database, String::from("1"), String::from("2"));

    get_jaccard_index_by_name(&simple_movies_database, String::from("Patrick C"), String::from("Heather"));
    get_jaccard_index_by_id(&simple_movies_database, String::from("1"), String::from("2"));
}
