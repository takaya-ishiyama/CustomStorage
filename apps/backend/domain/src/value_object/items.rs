use chrono::NaiveDateTime;

#[derive(Clone)]
pub struct Item {
    id: String,
    directory_id: String,
    texts: String,
    created_at: NaiveDateTime,
}

pub struct Items {
    items: Vec<Item>,
}

impl Item {
    pub fn new(id: String, directory_id: &str, texts: &str, created_at: NaiveDateTime) -> Self {
        Self {
            id,
            directory_id: directory_id.to_string(),
            texts: texts.to_string(),
            created_at,
        }
    }

    // FIXME: async garaphqlのために書いてるがあまり使いたくない
    pub fn get_properties(&self) -> (&str, &str, &str, &NaiveDateTime) {
        (&self.id, &self.directory_id, &self.texts, &self.created_at)
    }
}

impl Items {
    pub fn new(items: Vec<Item>) -> Self {
        Self { items }
    }
}
