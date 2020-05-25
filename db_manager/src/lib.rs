use std::collections::HashMap;

pub trait DBManager<U: User<I>, I: Item> {
    fn connect_to(url: &str) -> Self;

    fn get_user_by_name(&self, name: &str) -> Vec<U>;
    fn get_user_by_id(&self, uid: U::ID) -> Vec<U>;
    fn get_item_by_name(&self, name: &str) -> Vec<I>;
    fn get_item_by_id(&self, uid: U::ID) -> Vec<I>;
    fn get_all_users(&self) -> Vec<U>;
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