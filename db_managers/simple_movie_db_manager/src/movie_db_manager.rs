use diesel::prelude::*;
use diesel::pg::PgConnection;

use db_manager::DBManager;

use crate::{movie_user::MovieUser, movie_item::MovieItem, movie_rating};

pub struct MovieDBManager {
    connector:PgConnection
}

impl DBManager<MovieUser, MovieItem> for MovieDBManager {
    fn connect_to(url: &str) -> Self {
        let connector = PgConnection::establish(url).expect("Failed connection to database. Maybe the URL?");
        MovieDBManager{connector: connector}
    }

    fn get_user_by_name(&self, name: &str) -> Vec<MovieUser> {
        todo!()
    }
    fn get_user_by_id(&self, uid: u64) -> Vec<MovieUser> {
        todo!()
    }
    fn get_item_by_name(&self, name: &str) -> Vec<MovieItem> {
        todo!()
    }
    fn get_item_by_id(&self, uid: u64) -> Vec<MovieItem> {
        todo!()
    }
    fn get_all_users(&self) -> Vec<MovieUser> {
        todo!()
    }
}