mod session;
mod parser;

use std::sync::LazyLock;

use aes_gcm::{aead::{consts::U12, OsRng}, AeadCore, Aes256Gcm, KeyInit, Nonce};
use axum::Router;
use once_cell::sync::Lazy;
use sqlx::SqlitePool;
use tokio::net::TcpListener;

use crate::{parser::parse, session::{session_to_user, user_to_session}};

static DB: LazyLock<SqlitePool> = LazyLock::new(|| SqlitePool::connect_lazy("sqlite://db.sqlit").unwrap());

static CIPHER: Lazy<Aes256Gcm> = Lazy::new(|| Aes256Gcm::new(&Aes256Gcm::generate_key(OsRng)));
static NONCE: Lazy<Nonce<U12>> = Lazy::new(|| Aes256Gcm::generate_nonce(OsRng));

#[tokio::main]
async fn main() {
    dbg!(parse(String::from("abcABC123"), String::from("a=y")));
    /* let app = Router::new();

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap(); */
}
