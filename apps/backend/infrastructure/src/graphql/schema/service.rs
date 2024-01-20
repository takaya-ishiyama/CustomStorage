use async_graphql::SimpleObject;
use domain::value_object::{directory::Directory, items::Item, service::Service};

#[derive(SimpleObject)]
struct DirectorySchema {
    id: String,
    user_id: String,
    name: String,
    parent_id: Option<String>,
}

#[derive(SimpleObject)]
pub struct ItemSchema {
    id: String,
    directories_id: String,
    texts: String,
    // created_at: NaiveDateTime,
}

#[derive(SimpleObject)]
pub struct ServiceSchema {
    directories: Option<Vec<DirectorySchema>>,
    items: Option<Vec<ItemSchema>>,
}

impl DirectorySchema {
    pub fn new(directory: &Directory) -> Self {
        let _directory = directory.get_properties();
        Self {
            id: _directory.0.to_string(),
            user_id: _directory.1.to_string(),
            name: _directory.2.to_string(),
            parent_id: _directory.3.map(|parent_id| parent_id.to_string()),
        }
    }
}

impl ItemSchema {
    pub fn new(item: &Item) -> Self {
        let _item = item.get_properties();
        Self {
            id: _item.0.to_string(),
            directories_id: _item.1.to_string(),
            texts: _item.2.to_string(),
        }
    }
}

impl ServiceSchema {
    pub fn new(service: &Service) -> Self {
        Self {
            directories: service.get_directories().clone().map(|directories| {
                directories
                    .into_iter()
                    .map(|directory| DirectorySchema::new(&directory))
                    .collect()
            }),
            items: service.get_items().clone().map(|items| {
                items
                    .into_iter()
                    .map(|item| ItemSchema::new(&item))
                    .collect()
            }),
        }
    }
}
