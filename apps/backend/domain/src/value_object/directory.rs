use chrono::NaiveDateTime;

#[derive(Clone)]
pub struct Directory {
    id: String,
    user_id: String,
    name: String,
    parent_id: Option<String>,
    created_at: NaiveDateTime,
}

pub struct Directories {
    directories: Vec<Directory>,
}

impl Directory {
    pub fn new(
        id: String,
        user_id: String,
        name: String,
        parent_id: Option<String>,
        created_at: NaiveDateTime,
    ) -> Self {
        Self {
            id,
            user_id,
            name,
            parent_id,
            created_at,
        }
    }

    // FIXME: async garaphqlのために書いてるがあまり使いたくない
    pub fn get_properties(&self) -> (&str, &str, &str, Option<&str>, &NaiveDateTime) {
        (
            &self.id,
            &self.user_id,
            &self.name,
            self.parent_id.as_deref(),
            &self.created_at,
        )
    }
    pub fn not_exist_parent_id(&self) -> bool {
        if self.parent_id.is_none() {
            return true;
        }
        false
    }
    pub fn check_parent_id(&self, parent_id: &str) -> bool {
        if self.parent_id == Some(parent_id.to_string()) {
            return true;
        }
        false
    }

    /**テスト用。それ以外で使用してはいけない */
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /**テスト用。それ以外で使用してはいけない */
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /**テスト用。それ以外で使用してはいけない */
    pub fn get_parent_id(&self) -> Option<String> {
        self.parent_id.clone()
    }
}

impl Directories {
    pub fn new(directories: Vec<Directory>) -> Self {
        Self { directories }
    }
    pub fn find_by_pearent_id(&self, parent_id: Option<String>) -> Self {
        // let mut result: Vec<Directory> = Vec::new();
        match parent_id {
            Some(parent_id) => Self {
                directories: self
                    .directories
                    .iter()
                    .filter(|directory| directory.check_parent_id(&parent_id))
                    .cloned()
                    .collect(),
            },
            None => Self {
                directories: self
                    .directories
                    .iter()
                    .filter(|directory| directory.not_exist_parent_id())
                    .cloned()
                    .collect(),
            },
        }
    }
}
