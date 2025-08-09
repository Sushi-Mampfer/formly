mod session;
mod parser;
mod signup;
mod datatypes;
mod templates;
mod login;

use std::sync::LazyLock;

use aes_gcm::{aead::{consts::U12, OsRng}, AeadCore, Aes256Gcm, KeyInit, Nonce};
use axum::{routing::get, Router};
use rand::RngCore;
use sqlx::SqlitePool;
use tokio::net::TcpListener;

use crate::{datatypes::AppState, login::{login_api, login_page}, signup::{signup_api, signup_page}};

static SECRET: LazyLock<[u8; 32]> = LazyLock::new(|| {
    let mut buffer = [0u8; 32];
    let mut rng = rand::rng();
    rng.fill_bytes(&mut buffer);
    buffer
});

static CIPHER: LazyLock<Aes256Gcm> = LazyLock::new(|| Aes256Gcm::new(&Aes256Gcm::generate_key(OsRng)));
static NONCE: LazyLock<Nonce<U12>> = LazyLock::new(|| Aes256Gcm::generate_nonce(OsRng));

#[tokio::main]
async fn main() {
    let state = AppState {
        pool: SqlitePool::connect("sqlite://db.sqlite").await.unwrap()
    };

    let app = Router::new()
        .route("/signup", get(signup_page).post(signup_api))
        .route("/login", get(login_page).post(login_api))
        .with_state(state);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
