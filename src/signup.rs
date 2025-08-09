use std::borrow::Cow;

use askama::Template;
use axum::{extract::State, http::{header::{LOCATION, SET_COOKIE}, StatusCode}, response::{Html, IntoResponse, Response}, Form};
use sqlx::{error::DatabaseError, query, sqlite::SqliteError};

use crate::{datatypes::{AppState, SignUpData}, parser::parse, session::user_to_session, templates::SignUpPage};

pub async fn signup_api(state: State<AppState>, Form(data): Form<SignUpData>) -> Response {

    if parse(String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"), data.pattern.clone()) == None {
        return Html(SignUpPage { error: Some(String::from("Invalid pattern")) }.render().unwrap()).into_response()
    }

    let res = query("INSERT INTO users (name, pattern) VALUES (?, ?) RETURNING id")
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
        if let Some(session) = user_to_session(data.username, String::from("127.0.0.1")) {
            return (StatusCode::TEMPORARY_REDIRECT, [(LOCATION, "/dashboard"), (SET_COOKIE, &format!("session={}; Secure; HttpOnly; SameSite=Strict", session))]).into_response()
        } else {
            return Html(SignUpPage { error: Some(String::from("An unknown error occured")) }.render().unwrap()).into_response()
        }
    }

}
pub async fn signup_page() -> impl IntoResponse {
    Html(SignUpPage { error: None }.render().unwrap())
}