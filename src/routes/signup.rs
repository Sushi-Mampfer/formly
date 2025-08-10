use std::borrow::Cow;

use askama::Template;
use axum::{extract::State, http::{header::{LOCATION, SET_COOKIE}, HeaderMap, StatusCode}, response::{Html, IntoResponse, Redirect, Response}, Form};
use sqlx::{error::DatabaseError, query, sqlite::SqliteError};

use crate::{datatypes::{AppState, SignUpData}, parser::parse, session::{headers_to_user, user_to_session}, templates::SignUpPage};

pub async fn signup_api(state: State<AppState>, headers: HeaderMap, Form(data): Form<SignUpData>) -> Response {
    let ip = match headers.get("X-Forwarded-For").expect("X-Forwarded-For header not found.").to_str() {
        Ok(ip) => ip.to_string(),
        _ => return (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    };
    
    if let Some(_) = headers_to_user(headers) {
        return Redirect::to("/dashboard").into_response()
    }

    if parse(String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"), data.pattern.clone()) == None {
        return Html(SignUpPage { error: Some(String::from("Invalid pattern")) }.render().unwrap()).into_response()
    }

    let res = query("INSERT INTO users (username, pattern) VALUES (?, ?) RETURNING id")
        .bind(data.username.clone())
        .bind(data.pattern)
        .execute(&state.pool).await;
    let error =  match res {
        Ok(_) => None,
        Err(e) => {
            if let Some(e) = e.into_database_error() {
                if let Some(e) = e.downcast_ref::<SqliteError>().code() {
                    if e == Cow::from("2067") {
                        Some(String::from("Username is already taken"))
                    } else {
                        Some(String::from("An unknown error occured"))
                    }
                } else {
                    Some(String::from("An unknown error occured"))
                }
            } else {
                Some(String::from("An unknown error occured"))
            }
        }
    };
    if let Some(error) = error {    
       Html(SignUpPage { error: Some(error) }.render().unwrap()).into_response()
    } else {
        if let Some(session) = user_to_session(data.username, ip) {
            return (StatusCode::SEE_OTHER, [(LOCATION, "/dashboard"), (SET_COOKIE, &format!("session={}; Secure; HttpOnly; SameSite=Strict; Max-Age:={}", session, 60*60*24*7))]).into_response()
        } else {
            return Html(SignUpPage { error: Some(String::from("An unknown error occured")) }.render().unwrap()).into_response()
        }
    }

}
pub async fn signup_page(headers: HeaderMap) -> Response {
    if let Some(_) = headers_to_user(headers) {
        return Redirect::to("/dashboard").into_response()
    }

    Html(SignUpPage { error: None }.render().unwrap()).into_response()
}