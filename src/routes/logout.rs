use axum::{
    http::{
        StatusCode,
        header::{LOCATION, SET_COOKIE},
    },
    response::{IntoResponse, Response},
};

pub async fn logout() -> Response {
    (
        StatusCode::SEE_OTHER,
        [
            (LOCATION, "/login"),
            (
                SET_COOKIE,
                "session=deleted; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT",
            ),
        ],
    )
        .into_response()
}
