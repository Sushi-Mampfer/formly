mod datatypes;
mod parser;
mod routes;
mod session;
mod templates;

use std::{env, sync::LazyLock};

use aes_gcm::{
    AeadCore, Aes256Gcm, KeyInit, Nonce,
    aead::{OsRng, consts::U12},
};
use axum::Router;
use rand::RngCore;
use sqlx::SqlitePool;
use tokio::net::TcpListener;

use crate::{datatypes::AppState, routes::combind_routes};

static SECRET: LazyLock<[u8; 32]> = LazyLock::new(|| {
    let mut buffer = [0u8; 32];
    let mut rng = rand::rng();
    rng.fill_bytes(&mut buffer);
    buffer
});

static CIPHER: LazyLock<Aes256Gcm> =
    LazyLock::new(|| Aes256Gcm::new(&Aes256Gcm::generate_key(OsRng)));
static NONCE: LazyLock<Nonce<U12>> = LazyLock::new(|| Aes256Gcm::generate_nonce(OsRng));

#[tokio::main]
async fn main() {
    let state = AppState {
        pool: SqlitePool::connect("sqlite://db.sqlite").await.unwrap(),
    };

    let app = Router::new().merge(combind_routes(state));

    let listener = TcpListener::bind(format!(
        "127.0.0.1:{}",
        env::var("PORT").unwrap_or_else(|_| "8080".to_string())
    ))
    .await
    .unwrap();
    axum::serve(listener, app).await.unwrap();
}
