mod dashboard;
mod forms;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    datatypes::AppState,
    routes::dashboard::{
        dashboard::dashboard_page,
        forms::{create_api, create_page, delete_api, edit_api, edit_page, submissions_page},
    },
};

pub fn dashboard_router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/create", get(create_page).post(create_api))
        .route("/edit/{id}", get(edit_page).post(edit_api))
        .route("/submissions/{id}", get(submissions_page))
        .route("/delete/{id}", post(delete_api))
        .route("/", get(dashboard_page))
        .with_state(state)
}
