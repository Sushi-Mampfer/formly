use askama::Template;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
};
use futures::future::join_all;
use serde_json::from_str;
use sqlx::{Row, query};

use crate::{
    datatypes::{AppState, FormDefinition, FormShort},
    session::headers_to_user,
    templates::DashboardPage,
};

pub async fn dashboard_page(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let username = match headers_to_user(headers) {
        Some(u) => u,
        None => return Redirect::to("/login").into_response(),
    };
    let res = query("SELECT id, data FROM forms WHERE user = ?")
        .bind(&username)
        .fetch_all(&state.pool)
        .await;
    let rows = match res {
        Ok(r) => r,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    let forms = rows.iter().map(|row| async {
        let name = from_str::<FormDefinition>(row.get("data"))
            .map_err(|_| ())?
            .name;
        let id = row.get("id");
        let res = query("SELECT COUNT(*) as count FROM submissions WHERE form = ?")
            .bind(&id)
            .fetch_one(&state.pool)
            .await;
        let submissions: i32 = match res {
            Ok(r) => r.get("count"),
            Err(_) => return Err(()),
        };
        Ok(FormShort {
            id,
            name,
            submissions,
        })
    });
    let forms = match join_all(forms)
        .await
        .into_iter()
        .collect::<Result<Vec<FormShort>, _>>()
    {
        Ok(f) => f,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };
    Html(DashboardPage { username, forms }.render().unwrap()).into_response()
}
