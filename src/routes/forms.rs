use std::time::{SystemTime, UNIX_EPOCH};

use askama::Template;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use serde_json::{from_str, to_string};
use sqlx::{Error, Row, query};

use crate::{
    datatypes::{AppState, FormDefinition, FormSubmission},
    templates::FormPage,
};

pub async fn form_page(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    let res = query("SELECT data FROM forms WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.pool)
        .await;

    let row = match res {
        Ok(r) => r,
        Err(e) => match e {
            Error::RowNotFound => return StatusCode::NOT_FOUND.into_response(),
            _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    };
    let data = match from_str(row.get("data")) {
        Ok(d) => d,
        _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    Html(FormPage { id, data }.render().unwrap()).into_response()
}

pub async fn form_api(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(data): Json<FormSubmission>,
) -> Response {
    let res = query("SELECT data FROM forms WHERE id = ?")
        .bind(&id)
        .fetch_one(&state.pool)
        .await;

    let row = match res {
        Ok(r) => r,
        Err(e) => match e {
            Error::RowNotFound => return StatusCode::NOT_FOUND.into_response(),
            _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    };
    let layout: FormDefinition = match from_str(row.get("data")) {
        Ok(d) => d,
        _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let data_string = match to_string(&data) {
        Ok(d) => d,
        _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if !layout.validate_submission(data) {
        return StatusCode::BAD_REQUEST.into_response();
    }

    let res = query("INSERT INTO submissions (time, form, data) VALUES (?, ?, ?)")
        .bind(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
        )
        .bind(id)
        .bind(data_string)
        .execute(&state.pool)
        .await;

    match res {
        Ok(_) => (),
        _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    StatusCode::OK.into_response()
}
