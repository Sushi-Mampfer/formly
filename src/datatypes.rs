use std::sync::LazyLock;

use hmac::Hmac;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::SqlitePool;

pub static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
    r#" 	
(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#,
).unwrap()
});

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
                    if maxlen != 0 {
                        if data.len() > maxlen as usize {
                            return false;
                        }
                    }
                }
                (FieldKind::Email(name1), FieldValue::Email(name2, data)) => {
                    if name1 != name2 {
                        return false;
                    }
                    if !RE.is_match(&data) {
                        return false;
                    }
                }
                (FieldKind::Number(name1, min, max), FieldValue::Number(name2, data)) => {
                    if name1 != name2 {
                        return false;
                    }
                    if data < min || data > max {
                        return false;
                    }
                }
                (FieldKind::Multiple(name1, items), FieldValue::Multiple(name2, data)) => {
                    if name1 != name2 {
                        return false;
                    }
                    if !items.contains(&data) {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        true
    }
}
