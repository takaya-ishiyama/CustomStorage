use super::{directory::Directory, interface::service_interface::ServiceInterface, items::Item};

#[derive(Clone, Default)]
pub struct Service {
    directories: Option<Vec<Directory>>,
    items: Option<Vec<Item>>,
}

impl ServiceInterface for Service {
    fn new(directories: Option<Vec<Directory>>, items: Option<Vec<Item>>) -> Self {
        Self { directories, items }
    }

    // FIXME: async garaphqlのために書いてるがあまり使いたくない
    fn get_directories(&self) -> &Option<Vec<Directory>> {
        &self.directories
    }

    // FIXME: async garaphqlのために書いてるがあまり使いたくない
    fn get_items(&self) -> &Option<Vec<Item>> {
        &self.items
    }

    // pub fn get_properties(&self) -> (&Option<Vec<Directory>>, &Option<Vec<Item>>) {
    //     (&self.directories, &self.items)
    // }
}
