use askama::Template;
use axum::{
    Form,
    extract::State,
    http::{
        HeaderMap, StatusCode,
        header::{LOCATION, SET_COOKIE},
    },
    response::{Html, IntoResponse, Redirect, Response},
};
use hmac::Mac;
use rand::{Rng, SeedableRng, rngs::StdRng};
use sqlx::{Error::RowNotFound, Row, query};

use crate::{
    SECRET,
    datatypes::{ALPHANUMERIC, AppState, HmacSha256, LogInData},
    parser::parse,
    session::{headers_to_user, user_to_session},
    templates::LogInPage,
};

pub async fn login_api(
    state: State<AppState>,
    headers: HeaderMap,
    Form(data): Form<LogInData>,
) -> Response {
    let ip = match headers
        .get("X-Forwarded-For")
        .expect("X-Forwarded-For header not found.")
        .to_str()
    {
        Ok(ip) => ip.to_string(),
        _ => return (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    };

    if let Some(_) = headers_to_user(headers) {
        return Redirect::to("/dashboard").into_response();
    }

    let res = match query("SELECT pattern FROM users WHERE username = ?")
        .bind(data.username.clone())
        .fetch_one(&state.pool)
        .await
    {
        Ok(r) => r,
        Err(RowNotFound) => return create_login_template(Some(String::from("User not found"))),
        Err(_) => return create_login_template(Some(String::from("An unknown error occured"))),
    };
    if let Some(value) = parse(create_challenge(data.token), res.get("pattern")) {
        if value == data.answer {
            if let Some(session) = user_to_session(data.username, ip) {
                return (
                    StatusCode::SEE_OTHER,
                    [
                        (LOCATION, "/dashboard"),
                        (
                            SET_COOKIE,
                            &format!(
                                "session={}; Secure; HttpOnly; SameSite=Strict; Max-Age:={}",
                                session,
                                60 * 60 * 24 * 7
                            ),
                        ),
                    ],
                )
                    .into_response();
            } else {
                return create_login_template(Some(String::from("An unknown error occured")));
            }
        }
    }
    create_login_template(Some(String::from("Wrong answer")))
}

pub async fn login_page(headers: HeaderMap) -> Response {
    if let Some(_) = headers_to_user(headers) {
        return Redirect::to("/dashboard").into_response();
    }

    create_login_template(None)
}

pub fn create_login_template(error: Option<String>) -> Response {
    let mut rng = rand::rng();
    let token: String = (0..16)
        .map(|_| ALPHANUMERIC[rng.random_range(0..ALPHANUMERIC.len())])
        .collect();
    Html(
        LogInPage {
            error: error,
            token: token.clone(),
            challenge: create_challenge(token),
        }
        .render()
        .unwrap(),
    )
    .into_response()
}

pub fn create_challenge(token: String) -> String {
    let mut mac = HmacSha256::new_from_slice(&*SECRET).unwrap();
    mac.update(token.as_bytes());
    let hash: [u8; 32] = mac.finalize().into_bytes().into();
    let mut rng = StdRng::from_seed(hash);
    (0..8)
        .map(|_| ALPHANUMERIC[rng.random_range(0..ALPHANUMERIC.len())])
        .collect()
}
