#[macro_use]
extern crate diesel;

pub mod schema;

pub mod movie_user;
pub mod movie_item;
pub mod movie_rating;
pub mod small_movielens_db_manager;

#[cfg(test)]
mod tests {
    use super::small_movielens_db_manager::SmallMovielensDBManager;
    use db_manager::{DBManager};

    #[test]
    fn query_user() {
        let manager = SmallMovielensDBManager::connect_to("postgres://ademir:@localhost/small_movielens");

        let users = manager.get_user_by_name("Chris");
        
        println!("{:?}\n", users);

        let users = manager.get_user_by_id(2);
        
        println!("{:?}\n", users);

        //Dont do this, it takes a lot of time.. REFACTORING
        //let users = manager.get_all_users();
        
        //println!("{:?}\n", users);

        let movies = manager.get_item_by_name("Jumanji (1995)");
        
        println!("{:?}\n", movies);

        let movies = manager.get_item_by_id(2);
        
        println!("{:?}\n", movies);

        let ratings = manager.get_all_ratings();

        println!("{:?}\n", ratings[&2]);

        let users = manager.get_user_by_id(2);

        println!("{:?}\n", users);
    }
}