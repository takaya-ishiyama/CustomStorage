pub trait TokenInterface {
    fn new(access_token: String, refresh_token: String, expiration_timestamp: i64) -> Self;
    fn check_expiration(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
    pub expiration_timestamp: i64,
}

impl TokenInterface for Token {
    fn new(access_token: String, refresh_token: String, expiration_timestamp: i64) -> Self {
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
