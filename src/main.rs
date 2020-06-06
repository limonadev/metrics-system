use::std::fmt::Debug;

use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::hash::Hash;
use std::marker::PhantomData;
use std::cmp::{Reverse, min};

use db_manager::{DBManager, User, Item};
use simple_movie_db_manager::{movie_db_manager::MovieDBManager, movie_user::MovieUser, movie_item::MovieItem};
use book_db_manager::{book_db_manager::BookDBManager, book_user::BookUser, book_item::BookItem};
use small_movielens_db_manager::{small_movielens_db_manager::SmallMovielensDBManager, movie_user::SMovieLensUser, movie_item::SMovieLensItem};

pub mod simple_movie_interface;
pub mod small_movielens_interface;

#[derive(Clone,PartialEq)]
enum KNNMetric {
    Manhattan,
    Euclidean,
    Minkowski(i32),
    Pearson,
    Cosine,
    JaccardDistance,
    JaccardIndex
}

#[derive(Debug, Clone)]
struct PairDist<U> {
    id:U,
    value:f64
}

impl<U> Eq for PairDist<U> {}

impl<U> PartialEq for PairDist<U> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<U> PartialOrd for PairDist<U> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<U> Ord for PairDist<U> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.partial_cmp(&other.value).unwrap()
    }
}

pub struct Engine<U, I> {
    phantom_U: PhantomData<U>,
    phantom_I: PhantomData<I>
}

impl<U:Hash+Eq+Clone+Debug,I:Hash+Eq+Clone> Engine<U,I> {

    fn manhattan_distance_between(&self, first: &HashMap<I, f64>, second: &HashMap<I, f64>) -> f64 {
        Self::minkowski_distance_between(self, first, second, 1)
    }

    fn euclidean_distance_between(&self, first: &HashMap<I, f64>, second: &HashMap<I, f64>) -> f64 {
        Self::minkowski_distance_between(self, first, second, 2)
    }

    fn minkowski_distance_between(&self, first: &HashMap<I, f64>, second: &HashMap<I, f64>, grade: i32) -> f64 {
        let (smallest, biggest) = if first.len() < second.len() {
            (first, second)
        } else {
            (second, first)
        };

        let mut distance = 0.0;

        for (item_id, first_ranking) in smallest {
            if let Some(second_ranking) = biggest.get(item_id) {
                let diff = (first_ranking-second_ranking).abs().powi(grade);
                distance += diff;
            }
        }

        distance.powf(1.0/(grade as f64))
    }

    fn pearson_correlation_between(&self, first: &HashMap<I, f64>, second: &HashMap<I, f64>) -> f64 {
        let (smallest, biggest) = if first.len() < second.len() {
            (first, second)
        } else {
            (second, first)
        };

        let mut sum_x_by_y = 0.0;
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_x_squared = 0.0;
        let mut sum_y_squared = 0.0;
        let mut n = 0.0;
        for (item_id, first_ranking) in smallest {
            if let Some(second_ranking) = biggest.get(item_id) {
                sum_x_by_y += first_ranking*second_ranking;
                sum_x += first_ranking;
                sum_y += second_ranking;
                sum_x_squared += first_ranking.powi(2);
                sum_y_squared += second_ranking.powi(2);
                n += 1.0;
            }
        }

        let numerator = sum_x_by_y - ((sum_x*sum_y)/n);
        let first_root = (sum_x_squared - (sum_x.powi(2)/n)).sqrt();
        let second_root = (sum_y_squared - (sum_y.powi(2)/n)).sqrt();
        let denominator = first_root*second_root;

        numerator/denominator
    }

    fn cosine_similarity_between(&self, first: &HashMap<I, f64>, second: &HashMap<I, f64>) -> f64 {
        let (smallest, biggest) = if first.len() < second.len() {
            (first, second)
        } else {
            (second, first)
        };

        let mut first_len = 0.0;
        let mut second_len = 0.0;
        let mut pointwise_sum = 0.0;
        
        for (item_id, first_ranking) in smallest {
            if let Some(second_ranking) = biggest.get(item_id) {
                pointwise_sum += first_ranking*second_ranking;
                first_len += first_ranking.powi(2);
                second_len += second_ranking.powi(2);
            }
        }

        first_len = first_len.sqrt();
        second_len = second_len.sqrt();

        pointwise_sum/(first_len*second_len)
    }

    fn jaccard_index_between(&self, first: &HashMap<I, f64>, second: &HashMap<I, f64>) -> f64 {
        let (smallest, biggest) = if first.len() < second.len() {
            (first, second)
        } else {
            (second, first)
        };

        let mut intersection = 0;
        for item_id in smallest.keys() {
            if biggest.contains_key(item_id){
                intersection += 1;
            }
        }
        let union = (smallest.keys().len() - intersection) + (biggest.keys().len() - intersection) + intersection;
        intersection as f64/union as f64
    }

    fn jaccard_distance_between(&self, first: &HashMap<I, f64>, second: &HashMap<I, f64>) -> f64 {
        let dist  = Self::jaccard_index_between(self, first, second);
        1.0 - dist
    }

    fn pearson_cosine_jac_in_nearest_neighbors(&self, k:i32, target_id:U, target_ratings:&HashMap<I, f64>, ratings:&HashMap<U,HashMap<I,f64>>, metric: &KNNMetric) -> Vec<PairDist<U>> {
        let mut min_heap = BinaryHeap::new();

        let mut ratings_without_user = ratings.clone();
        if ratings_without_user.contains_key(&target_id) {
            ratings_without_user.remove(&target_id);
        }

        for (u, u_ratings) in &ratings_without_user {
            let dist = match metric {
                KNNMetric::Manhattan => {f64::NAN}
                KNNMetric::Euclidean => {f64::NAN}
                KNNMetric::Minkowski(_) => {f64::NAN}
                KNNMetric::Pearson => {
                    Self::pearson_correlation_between(self, target_ratings, u_ratings)
                }
                KNNMetric::Cosine => {
                    Self::cosine_similarity_between(self, target_ratings, u_ratings)
                }
                KNNMetric::JaccardDistance => {f64::NAN}
                KNNMetric::JaccardIndex => {
                    Self::jaccard_index_between(self, target_ratings, u_ratings)
                }
            };

            if dist == f64::NAN || dist == f64::INFINITY || dist == -f64::INFINITY || dist.is_nan() {
                continue;
            }

            let pair_dist = PairDist::<U> {id: u.clone(), value: dist};
            if min_heap.len() < k as usize {
                min_heap.push(Reverse(pair_dist));
            } else {
                if min_heap.peek().unwrap().0.value < pair_dist.value {
                    min_heap.pop();
                    min_heap.push(Reverse(pair_dist));
                }
            }
        }

        let mut k_neighbors = Vec::new();
        while min_heap.len() > 0 {
            let pair = min_heap.peek().unwrap();
            k_neighbors.push(PairDist::<U>{id: pair.0.id.clone(), value:pair.0.value});
            min_heap.pop();
        }
        k_neighbors.reverse();
        k_neighbors
    }

    fn k_nearest_neighbors(&self, k:i32, target_id:U, target_ratings:&HashMap<I, f64>, ratings:&HashMap<U,HashMap<I,f64>>, metric: &KNNMetric) -> Vec<PairDist<U>>{
        if KNNMetric::Pearson == *metric || KNNMetric::Cosine == *metric || KNNMetric::JaccardIndex == *metric{
            return Self::pearson_cosine_jac_in_nearest_neighbors(self, k, target_id, target_ratings, ratings, metric);
        }

        let mut max_heap:BinaryHeap<PairDist<U>> = BinaryHeap::new();

        let mut ratings_without_user = ratings.clone();
        if ratings_without_user.contains_key(&target_id) {
            ratings_without_user.remove(&target_id);
        }

        for (u, u_ratings) in &ratings_without_user {
            let dist = match metric {
                KNNMetric::Manhattan => {Self::manhattan_distance_between(self, target_ratings, u_ratings)}
                KNNMetric::Euclidean => {Self::euclidean_distance_between(self, target_ratings, u_ratings)}
                KNNMetric::Minkowski(grade) => {Self::minkowski_distance_between(self, target_ratings, u_ratings, *grade)}
                KNNMetric::Pearson => {f64::NAN},
                KNNMetric::Cosine => {f64::NAN}
                KNNMetric::JaccardDistance => {Self::jaccard_distance_between(self, target_ratings, u_ratings)}
                KNNMetric::JaccardIndex => {f64::NAN}
            };

            if dist == f64::NAN || dist == f64::INFINITY || dist == -f64::INFINITY || dist.is_nan() {
                continue;
            }

            let pair_dist = PairDist::<U> {id: u.clone(), value: dist};
            if max_heap.len() < k as usize {
                max_heap.push(pair_dist);
            } else {
                if max_heap.peek().unwrap() > &pair_dist {
                    max_heap.pop();
                    max_heap.push(pair_dist);
                }
            }
        }

        let mut k_neighbors = Vec::new();
        while max_heap.len() > 0 {
            let pair = max_heap.peek().unwrap();
            k_neighbors.push(PairDist::<U>{id: pair.id.clone(), value:pair.value});
            max_heap.pop();
        }
        k_neighbors.reverse();
        k_neighbors
    }
}

pub struct Auxiliar<U, I> {
    phantom_U: PhantomData<U>,
    phantom_I: PhantomData<I>
}

impl<U:Hash+Eq+Clone+Debug,I:Hash+Eq+Clone> Auxiliar<U,I> {
    fn merge_min_heap(&self, k: i32, first_heap: &Vec<PairDist<U>>, second_heap: &Vec<PairDist<U>>) -> Vec<PairDist<U>> {
        let mut first_heap = first_heap.to_vec();
        first_heap.reverse();
        let mut second_heap = second_heap.to_vec();
        second_heap.reverse();

        let mut sorted = first_heap;
        sorted.extend(second_heap);

        sorted.sort();
        sorted.reverse();
        
        let mut result = Vec::new();
        for i in 0..min(k, sorted.len() as i32) {
            result.push(sorted[i as usize].clone());
        }
        result
    }

    fn merge_max_heap(&self, k: i32, first_heap: &Vec<PairDist<U>>, second_heap: &Vec<PairDist<U>>) -> Vec<PairDist<U>> {
        let mut sorted = first_heap.to_vec();
        sorted.extend(second_heap.iter().cloned());

        sorted.sort();
        
        let mut result = Vec::new();
        for i in 0..min(k, sorted.len() as i32) {
            result.push(sorted[i as usize].clone());
        }
        result
    }

    fn merge_heap_results_for_knn(&self, k: i32, first_heap: &Vec<PairDist<U>>, second_heap: &Vec<PairDist<U>>, metric: &KNNMetric) -> Vec<PairDist<U>> {
        match metric {
            KNNMetric::Manhattan | KNNMetric::Euclidean | KNNMetric::Minkowski(_) | KNNMetric::JaccardDistance => {
                self.merge_max_heap(k, first_heap, second_heap)
            }
            KNNMetric::Pearson | KNNMetric::Cosine | KNNMetric::JaccardIndex => {
                self.merge_min_heap(k, first_heap, second_heap)
            }
        }
    }
}


fn small_movielens_knn(k: i32, target_id: String, metric: KNNMetric) {
    let manager = SmallMovielensDBManager::connect_to("postgres://ademir:@localhost/small_movielens");
    let engine = Engine::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};
    let auxiliar = Auxiliar::<i32,i32> {phantom_U: PhantomData, phantom_I: PhantomData};

    let target_id = target_id.parse().expect("Failed to parse target id in K Neighbors");
    let target_user = manager.get_user_by_id(target_id);
    if target_user.is_empty() {
        println!("No user with id {} found!", target_id);
        return;
    }

    let target_user = target_user[0].clone();
    let target_ratings = target_user.ratings();

    const USERS_NUMBER:i64 = 610;
    let chunk_size:i64 = k as i64;

    let mut nearest_neighbors = Vec::new();

    for chunk_it in (0..USERS_NUMBER).step_by(chunk_size as usize) {
        let users_with_ratings = manager.get_users_with_ratings_chunk(chunk_it, chunk_size);

        let chunk_knn = engine.k_nearest_neighbors(k, target_id, &target_ratings, &users_with_ratings, &metric);
        nearest_neighbors = auxiliar.merge_heap_results_for_knn(k, &nearest_neighbors, &chunk_knn, &metric);
    }

    let mut users = Vec::new();
    for neighbor in &nearest_neighbors {
        users.push(neighbor.id);
    }

    for n in &nearest_neighbors {
        println!("{} {}", n.id, n.value);
    }

}

fn main() {
    //small_movielens_knn(500, String::from("1"), KNNMetric::Manhattan);
    let (simple_movie_order, simple_movie_matrix) = simple_movie_interface::get_similarity_matrix();
    let (small_movielens_order, small_movielens_matrix) = small_movielens_interface::get_similarity_matrix();

    let similarity = simple_movie_interface::get_similarity_between(
        &simple_movie_order, &simple_movie_matrix,
        Some(String::from("Alien")), None,
        Some(String::from("Avatar")), None
    ).expect("Failed to get the similarity");

    println!("{}", similarity);

    let similarity = simple_movie_interface::get_similarity_between(
        &simple_movie_order, &simple_movie_matrix,
        Some(String::from("Braveheart")), None,
        Some(String::from("The Happening")), None
    ).expect("Failed to get the similarity");

    println!("{}", similarity);

    let similarity = simple_movie_interface::get_similarity_between(
        &simple_movie_order, &simple_movie_matrix,
        Some(String::from("Pulp Fiction")), None,
        Some(String::from("Toy Story")), None
    ).expect("Failed to get the similarity");

    println!("{}", similarity);

    let similarity = simple_movie_interface::get_similarity_between(
        &simple_movie_order, &simple_movie_matrix,
        Some(String::from("Star Wars")), None,
        Some(String::from("Jaws")), None
    ).expect("Failed to get the similarity");

    println!("{}", similarity);

    println!("\n");

    let similarity = small_movielens_interface::get_similarity_between(
        &small_movielens_order, &small_movielens_matrix,
        Some(String::from("Iron Will (1994)")), None,
        Some(String::from("Spider-Man (2002)")), None
    ).expect("Failed to get the similarity");

    println!("{}", similarity);

    let similarity = small_movielens_interface::get_similarity_between(
        &small_movielens_order, &small_movielens_matrix,
        Some(String::from("Micmacs (Micmacs à tire-larigot) (2009)")), None,
        Some(String::from("Room, The (2003)")), None
    ).expect("Failed to get the similarity");

    println!("{}", similarity);

    let similarity = small_movielens_interface::get_similarity_between(
        &small_movielens_order, &small_movielens_matrix,
        Some(String::from("Sabrina (1995)")), None,
        Some(String::from("Casino (1995)")), None
    ).expect("Failed to get the similarity");

    println!("{}", similarity);

    let similarity = small_movielens_interface::get_similarity_between(
        &small_movielens_order, &small_movielens_matrix,
        Some(String::from("Dangerous Minds (1995)")), None,
        Some(String::from("Friday (1995)")), None
    ).expect("Failed to get the similarity");

    println!("{}", similarity);

}

/*
let mut all_ratings = HashMap::new();
let mut ratings_user_1 = HashMap::new();
ratings_user_1.insert(2, 3.0);
ratings_user_1.insert(3, 5.0);
ratings_user_1.insert(4, 4.0);
ratings_user_1.insert(5, 1.0);
all_ratings.insert(1, ratings_user_1);

let mut ratings_user_2 = HashMap::new();
ratings_user_2.insert(2, 3.0);
ratings_user_2.insert(3, 4.0);
ratings_user_2.insert(4, 4.0);
ratings_user_2.insert(5, 1.0);
all_ratings.insert(2, ratings_user_2);

let mut ratings_user_3 = HashMap::new();
ratings_user_3.insert(1, 4.0);
ratings_user_3.insert(2, 3.0);
ratings_user_3.insert(4, 3.0);
ratings_user_3.insert(5, 1.0);
all_ratings.insert(3, ratings_user_3);

let mut ratings_user_4 = HashMap::new();
ratings_user_4.insert(1, 4.0);
ratings_user_4.insert(2, 4.0);
ratings_user_4.insert(3, 4.0);
ratings_user_4.insert(4, 3.0);
ratings_user_4.insert(5, 1.0);
all_ratings.insert(4, ratings_user_4);

let mut ratings_user_5 = HashMap::new();
ratings_user_5.insert(1, 5.0);
ratings_user_5.insert(2, 4.0);
ratings_user_5.insert(3, 5.0);
ratings_user_5.insert(5, 3.0);
all_ratings.insert(5, ratings_user_5);
*/