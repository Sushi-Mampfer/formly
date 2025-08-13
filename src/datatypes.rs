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
pub struct FormDefinition {
    pub name: String,
    pub fields: Vec<FieldKind>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum FieldKind {
    Text(String, i32),
    Email(String),
    Number(String, i32, i32),
    Multiple(String, Vec<String>),
}

#[derive(Serialize, Deserialize)]
pub struct FormSubmission {
    pub values: Vec<FieldValue>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum FieldValue {
    Text(String, String),
    Email(String, String),
    Number(String, i32),
    Multiple(String, String),
}

impl FormDefinition {
    pub fn validate_submission(self, submission: FormSubmission) -> bool {
        if self.fields.len() != submission.values.len() {
            return false;
        }
        for i in 0..self.fields.len() {
            match (self.fields[i].clone(), submission.values[i].clone()) {
                (FieldKind::Text(name1, maxlen), FieldValue::Text(name2, data)) => {
                    if name1 != name2 {
                        return false;
                    }
                    if data.len() > maxlen as usize {
                        return false;
                    }
                }
                (FieldKind::Email(name1), FieldValue::Email(name2, data)) => {
                    if name1 != name2 {
                        return false;
                    }
                }
                (FieldKind::Number(name1, _, _), FieldValue::Number(name2, _)) => {
                    if name1 != name2 {
                        return false;
                    }
                }
                (FieldKind::Multiple(name1, items), FieldValue::Multiple(name2, _)) => {
                    if name1 != name2 {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        true
    }
}
