use uuid::Uuid;

pub trait SessionInterface {
    fn new(access_token: String, refresh_token: String, expiration_timestamp: i64) -> Self;
    fn check_expiration(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Session {
    pub access_token: String,
    pub refresh_token: String,
    pub expiration_timestamp: i64,
}

impl SessionInterface for Session {
    fn new(access_token: String, refresh_token: String, expiration_timestamp: i64) -> Self {
        let access_token = if access_token.is_empty() {
            generate_token()
        } else {
            access_token
        };
        let refresh_token = if refresh_token.is_empty() {
            generate_token()
        } else {
            refresh_token
        };
        Self {
            access_token,
            refresh_token,
            expiration_timestamp,
        }
    }
    fn check_expiration(&self) -> bool {
        let now = chrono::Local::now().timestamp();
        if now > self.expiration_timestamp {
            return false;
        }
        true
    }
}

fn generate_token() -> String {
    Uuid::new_v4().to_string()
}
