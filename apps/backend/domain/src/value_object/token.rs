use chrono::{DateTime, Local, NaiveDateTime};
use uuid::Uuid;

pub trait SessionInterface {
    fn new(
        user_id: &str,
        access_token: &str,
        refresh_token: &str,
        expiration_timestamp: &NaiveDateTime,
    ) -> Self;
    fn create(&self) -> Self;
    fn check_expiration(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Session {
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expiration_timestamp: NaiveDateTime,
}

impl SessionInterface for Session {
    fn new(
        user_id: &str,
        access_token: &str,
        refresh_token: &str,
        expiration_timestamp: &NaiveDateTime,
    ) -> Self {
        Self {
            user_id: user_id.to_string(),
            access_token: access_token.to_string(),
            refresh_token: refresh_token.to_string(),
            expiration_timestamp: *expiration_timestamp,
        }
    }
    fn create(&self) -> Self {
        let access_token = generate_token();
        let refresh_token = generate_token();
        let expiration_timestamp = Local::now()
            .naive_local()
            .checked_add_signed(chrono::Duration::hours(1))
            .unwrap();

        Self {
            user_id: self.user_id.clone(),
            access_token,
            refresh_token,
            expiration_timestamp,
        }
    }
    fn check_expiration(&self) -> bool {
        let now = chrono::Local::now().naive_local();
        if now > self.expiration_timestamp {
            return false;
        }
        true
    }
}

fn generate_token() -> String {
    Uuid::new_v4().to_string()
}
