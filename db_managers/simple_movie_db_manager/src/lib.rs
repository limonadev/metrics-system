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

        let users = manager.get_user_by_name("Chris");
        
        println!("{:?}\n", users);

        let users = manager.get_user_by_id(10);
        
        println!("{:?}\n", users);

        let users = manager.get_all_users();
        
        println!("{:?}\n", users);

        let movies = manager.get_item_by_name("Avatar");
        
        println!("{:?}\n", movies);

        let movies = manager.get_item_by_id(2);
        
        println!("{:?}\n", movies);


        //Testing the new get_all_ratings() versus the result obtained by get_user_by_id()
        let ratings = manager.get_all_ratings();
        
        println!("{:?}\n", ratings[&21]);

        let users = manager.get_user_by_id(21);
        
        println!("{:?}\n", users);
    }
}