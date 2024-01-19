use super::{directory::Directory, items::Item};

#[derive(Clone, Default)]
pub struct Service {
    directories: Option<Vec<Directory>>,
    items: Option<Vec<Item>>,
}

impl Service {
    pub fn new(directories: Option<Vec<Directory>>, items: Option<Vec<Item>>) -> Self {
        Self { directories, items }
    }
}
