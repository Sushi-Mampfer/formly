use askama::Template;
use axum::response::{Html, IntoResponse, Response};

use crate::templates::IndexPage;

pub async fn index_page() -> Response {
    Html(IndexPage {}.render().unwrap()).into_response()
}