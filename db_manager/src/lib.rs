use std::collections::HashMap;

pub trait DBManager<U: User<I>, I: Item> {
    fn connect_to(url: &str) -> Self;

    fn get_user_by_name(&self, name: &str) -> Vec<U>;
    fn get_user_by_id(&self, uid: U::ID) -> Vec<U>;
    fn get_item_by_name(&self, name: &str) -> Vec<I>;
    fn get_item_by_id(&self, uid: I::ID) -> Vec<I>;
    fn get_all_users(&self) -> Vec<U>;
    fn get_all_ratings(&self) -> HashMap<U::ID, HashMap<I::ID, f64>>;
    fn get_users_chunk(&self, offset: i64, limit: i64) -> Vec<U::ID>;
    fn get_user_ratings(&self, uid: U::ID) -> HashMap<I::ID, f64>;
    fn get_users_with_ratings_chunk(&self, offset: i64, limit: i64) -> HashMap<U::ID, HashMap<I::ID, f64>>;
}

pub trait User<I: Item> {
    type ID;

    fn id(&self) -> Self::ID;
    fn name(&self) -> String;
    fn data(&self) -> HashMap<String, String>;
    fn ratings(&self) -> HashMap<I::ID, f64>;
}

pub trait Item {
    type ID;

    fn id(&self) -> Self::ID;
    fn name(&self) -> String;
    fn data(&self) -> HashMap<String, String>;
}