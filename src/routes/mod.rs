mod dashboard;
mod forms;
mod index;
mod login;
mod logout;
mod signup;

use axum::{Router, response::Redirect, routing::get};
use tower_http::services::ServeDir;

use crate::{
    datatypes::AppState,
    routes::{
        dashboard::dashboard_router,
        forms::{form_api, form_page},
        index::index_page,
        login::{login_api, login_page},
        logout::logout,
        signup::{signup_api, signup_page},
    },
};

pub fn combind_routes(state: AppState) -> Router {
    Router::new()
        .route("/signup", get(signup_page).post(signup_api))
        .route("/logout", get(logout))
        .route("/login", get(login_page).post(login_api))
        .route("/form/{id}", get(form_page).post(form_api))
        .route("/", get(index_page))
        .route(
            "/dashboard/",
            get(|| async { Redirect::permanent("/dashboard") }),
        )
        .nest("/dashboard", dashboard_router(state.clone()))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state)
}
