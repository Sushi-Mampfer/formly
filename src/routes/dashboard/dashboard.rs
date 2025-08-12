use askama::Template;
use axum::{
    http::HeaderMap,
    response::{Html, IntoResponse, Redirect, Response},
};

use crate::{session::headers_to_user, templates::DashboardPage};

pub async fn dashboard_page(headers: HeaderMap) -> Response {
    if let Some(username) = headers_to_user(headers) {
        Html(DashboardPage { username }.render().unwrap()).into_response()
    } else {
        Redirect::to("/login").into_response()
    }
}
