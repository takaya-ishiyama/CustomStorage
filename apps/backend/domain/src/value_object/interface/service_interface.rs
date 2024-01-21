use crate::value_object::{directory::Directory, items::Item};

pub trait ServiceInterface {
    fn new(directories: Option<Vec<Directory>>, items: Option<Vec<Item>>) -> Self;
    fn get_directories(&self) -> &Option<Vec<Directory>>;
    fn get_items(&self) -> &Option<Vec<Item>>;
}
