use askama::{Template};
use axum::{extract::State, http::{header::{LOCATION, SET_COOKIE}, HeaderMap, StatusCode}, response::{Html, IntoResponse, Redirect, Response}, Form};
use hmac::{Mac};
use rand::{rngs::StdRng, Rng, SeedableRng};
use sqlx::{Row, query, Error::RowNotFound};

use crate::{datatypes::{AppState, HmacSha256, LogInData, ALPHANUMERIC}, parser::parse, session::user_to_session, templates::LogInPage, SECRET};

pub async fn login_api(state: State<AppState>, header: HeaderMap, Form(data): Form<LogInData>) -> Response {
    let res = match query("SELECT pattern FROM users WHERE name = ?")
        .bind(data.username.clone())
        .fetch_one(&state.pool).await {
            Ok(r) => r,
            Err(RowNotFound) => return create_login_template(Some(String::from("User not found"))),
            Err(_) => return create_login_template(Some(String::from("An unknown error occured")))
        };
    if let Some(value) = parse(create_challenge(data.token), res.get("pattern")) {
        if value == data.answer {
            if let Some(session) = user_to_session(data.username, String::from("127.0.0.1")) {
                return (StatusCode::TEMPORARY_REDIRECT, [(LOCATION, "/dashboard"), (SET_COOKIE, &format!("session={}; Secure; HttpOnly; SameSite=Strict", session))]).into_response()
            } else {
                return create_login_template(Some(String::from("An unknown error occured")))
            }
        }
    }
    create_login_template(Some(String::from("Wrong answer")))
}

pub async fn login_page() -> Response {
    create_login_template(None)
}

pub fn create_login_template(error: Option<String>) -> Response {
    let mut rng = rand::rng();
    let token: String = (0..16).map(|_| ALPHANUMERIC[rng.random_range(0..ALPHANUMERIC.len())]).collect();
    Html(LogInPage {
        error: error,
        token: token.clone(),
        challenge: create_challenge(token)
    }.render().unwrap()).into_response()
}

pub fn create_challenge(token: String) -> String {
    let mut mac = HmacSha256::new_from_slice(&*SECRET).unwrap();
    mac.update(token.as_bytes());
    let hash: [u8; 32] = mac.finalize().into_bytes().into();
    let mut rng = StdRng::from_seed(hash);
    (0..8).map(|_| ALPHANUMERIC[rng.random_range(0..ALPHANUMERIC.len())]).collect()
}