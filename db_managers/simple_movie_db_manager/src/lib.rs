#[macro_use]
extern crate diesel;

pub mod schema;

pub mod movie_user;
pub mod movie_item;
pub mod movie_rating;
pub mod movie_db_manager;

#[cfg(test)]
mod tests {
    use super::movie_db_manager::MovieDBManager;
    use db_manager::{DBManager};

    #[test]
    fn query_user() {
        let manager = MovieDBManager::connect_to("postgres://ademir:@localhost/simple_movies");

        let user = manager.get_user_by_name("Chris");
        
        println!("{:?}", user);

        let user = manager.get_user_by_id(15);
        
        println!("{:?}", user);
    }
}