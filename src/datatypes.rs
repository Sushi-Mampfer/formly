use hmac::Hmac;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::SqlitePool;

pub static ALPHANUMERIC: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

pub type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

#[derive(Serialize, Deserialize)]
pub struct SignUpData {
    pub username: String,
    pub pattern: String,
}

#[derive(Serialize, Deserialize)]
pub struct LogInData {
    pub username: String,
    pub token: String,
    pub answer: String,
}

#[derive(Serialize, Deserialize)]
pub struct FormData {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Serialize, Deserialize)]
pub enum Field {
    TextField(TextField),
    NumberField(NumberField),
    EmailField(EmailField),
    MultipleField(MultipleField),
}

#[derive(Serialize, Deserialize)]
pub struct TextField {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct NumberField {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct EmailField {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MultipleField {
    pub name: String,
    pub choices: Vec<String>,
}
