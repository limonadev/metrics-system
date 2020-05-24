use std::collections::HashMap;

pub trait DBManager<U: User, I: Item> {
    fn connect_to(url: &str) -> Self;

    fn get_user_by_name(&self, name: &str) -> Vec<U>;
    fn get_user_by_id(&self, uid: u64) -> Vec<U>;
    fn get_item_by_name(&self, name: &str) -> Vec<I>;
    fn get_item_by_id(&self, uid: u64) -> Vec<I>;
    fn get_all_users(&self) -> Vec<U>;
}

pub trait User {
    fn id(&self) -> u64;
    fn name(&self) -> String;
    fn data(&self) -> HashMap<String, String>;
    fn ratings(&self) -> HashMap<u64, f64>;
}

pub trait Item {
    fn id(&self) -> u64;
    fn name(&self) -> String;
    fn data(&self) -> HashMap<String, String>;
}