mod login;
mod logout;
mod signup;
mod dashboard;
mod index;

use axum::{routing::get, Router};

use crate::{datatypes::AppState, routes::{dashboard::dashboard_page, index::index_page, login::{login_api, login_page}, logout::logout, signup::{signup_api, signup_page}}};

pub fn combind_routes(state: AppState) -> Router {
    Router::new()
        .route("/signup", get(signup_page).post(signup_api))
        .route("/logout", get(logout))
        .route("/login", get(login_page).post(login_api))
        .route("/dashboard", get(dashboard_page))
        .route("/", get(index_page))
        .with_state(state)
}