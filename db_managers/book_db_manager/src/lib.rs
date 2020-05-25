#[macro_use]
extern crate diesel;

pub mod schema;

pub mod book_user;
pub mod book_item;
pub mod book_rating;
pub mod book_db_manager;

#[cfg(test)]
mod tests {
    use super::book_db_manager::BookDBManager;
    use db_manager::{DBManager};

    #[test]
    fn query_user() {
        let manager = BookDBManager::connect_to("postgres://ademir:@localhost/books");

        let users = manager.get_user_by_name("Chris");
        
        println!("{:?}\n", users);

        let users = manager.get_user_by_id(2);
        
        println!("{:?}\n", users);

        //Dont do this, it takes a lot of time.. REFACTORING
        //let users = manager.get_all_users();
        
        //println!("{:?}\n", users);

        let books = manager.get_item_by_name("Avatar");
        
        println!("{:?}\n", books);

        let books = manager.get_item_by_id(String::from("adsa"));
        
        println!("{:?}\n", books);
    }
}