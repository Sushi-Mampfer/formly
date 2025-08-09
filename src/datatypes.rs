use hmac::Hmac;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::SqlitePool;

pub static ALPHANUMERIC: &[char] = &[
    'A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z',
    'a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z',
    '0','1','2','3','4','5','6','7','8','9',
];

pub type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool
}

#[derive(Serialize, Deserialize)]
pub struct SignUpData {
    pub username: String,
    pub pattern: String
}

#[derive(Serialize, Deserialize)]
pub struct LogInData {
    pub username: String,
    pub token: String,
    pub answer: String
}