use super::{directory::Directory, items::Item};

#[derive(Clone, Default)]
pub struct Service {
    directories: Vec<Directory>,
    items: Vec<Item>,
}

impl Service {
    pub fn new(directories: Vec<Directory>, items: Vec<Item>) -> Self {
        Self { directories, items }
    }
}
