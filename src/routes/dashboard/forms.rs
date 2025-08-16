use std::borrow::Cow;

use askama::Template;
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
};
use rand::{
    distr::{Alphabetic, SampleString},
    rng,
};
use serde_json::{from_str, to_string};
use sqlx::{Error, Row, error::DatabaseError, query, sqlite::SqliteError};

use crate::{
    datatypes::{AppState, FormDefinition, FormSubmission},
    session::headers_to_user,
    templates::{CreateFormPage, EditFormPage, SubmissionsPage},
};

pub async fn create_page(headers: HeaderMap) -> Response {
    if let Some(_username) = headers_to_user(headers) {
        Html(CreateFormPage {}.render().unwrap()).into_response()
    } else {
        Redirect::to("/login").into_response()
    }
}

pub async fn create_api(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(data): Json<FormDefinition>,
) -> Response {
    let username = match headers_to_user(headers) {
        Some(u) => u,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };
    let data = match to_string(&data) {
        Ok(s) => s,
        _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let mut id = String::new();

    for _ in 0..100 {
        id = Alphabetic.sample_string(&mut rng(), 10);
        let res = query("INSERT INTO forms (id, user, data) VALUES (?, ?, ?)")
            .bind(&id)
            .bind(&username)
            .bind(&data)
            .execute(&state.pool)
            .await;
        match res {
            Ok(_) => break,
            Err(e) => {
                if let Some(e) = e.into_database_error() {
                    if let Some(e) = e.downcast_ref::<SqliteError>().code() {
                        if e == Cow::from("2067") {
                            return StatusCode::INSUFFICIENT_STORAGE.into_response();
                        } else {
                            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                        }
                    } else {
                        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    }
                } else {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            }
        }
    }
    (StatusCode::OK, id).into_response()
}

pub async fn edit_page(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let username = match headers_to_user(headers) {
        Some(u) => u,
        None => return Redirect::to("/login").into_response(),
    };

    let res = query("SELECT data FROM forms WHERE id = ? AND user = ?")
        .bind(&id)
        .bind(username)
        .fetch_one(&state.pool)
        .await;
    let row = match res {
        Ok(r) => r,
        Err(sqlx::Error::RowNotFound) => return Redirect::to("/login").into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let data = match from_str(row.get("data")) {
        Ok(s) => s,
        _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    EditFormPage { id, data }.render().unwrap().into_response()
}

pub async fn edit_api(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(data): Json<FormDefinition>,
) -> Response {
    let username = match headers_to_user(headers) {
        Some(u) => u,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let data = match to_string(&data) {
        Ok(s) => s,
        _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let res = query("UPDATE forms SET data = ? WHERE id = ? AND user = ?")
        .bind(data)
        .bind(id)
        .bind(username)
        .execute(&state.pool)
        .await;

    let rows = match res {
        Ok(r) => r.rows_affected(),
        _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if rows == 0 {
        return StatusCode::NOT_FOUND.into_response();
    }

    return StatusCode::OK.into_response();
}

pub async fn submissions_page(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    let username = match headers_to_user(headers) {
        Some(u) => u,
        None => return Redirect::to("/login").into_response(),
    };

    let res = query("SELECT data FROM forms WHERE id = ? AND user = ?")
        .bind(&id)
        .bind(&username)
        .fetch_one(&state.pool)
        .await;

    let data = match res {
        Ok(r) => r.get::<String, _>("data"),
        Err(Error::RowNotFound) => return StatusCode::NOT_FOUND.into_response(),
        _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let data: FormDefinition = match from_str(&data) {
        Ok(d) => d,
        _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let name = data.name;

    let res = query("SELECT data FROM submissions WHERE form = ? ORDER BY time DESC")
        .bind(id)
        .fetch_all(&state.pool)
        .await;

    let rows = match res {
        Ok(r) => r,
        _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let data: Vec<FormSubmission> = match rows
        .iter()
        .map(|r| from_str(r.get("data")))
        .collect::<Result<_, _>>()
    {
        Ok(s) => s,
        _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    Html(SubmissionsPage { name, data }.render().unwrap()).into_response()
}
