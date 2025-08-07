mod session;

use aes_gcm::{aead::{consts::U12, OsRng}, AeadCore, Aes256Gcm, KeyInit, Nonce};
use once_cell::sync::Lazy;

use crate::session::{session_to_user, user_to_session};


static CIPHER: Lazy<Aes256Gcm> = Lazy::new(|| Aes256Gcm::new(&Aes256Gcm::generate_key(OsRng)));
static NONCE: Lazy<Nonce<U12>> = Lazy::new(|| Aes256Gcm::generate_nonce(OsRng));

fn main() {
    let ip1 = String::from("0000:0000:0000:0000:0000:0000:0000:0000");
    let ip2 = String::from("127.0.0.2");
    let username = String::from("asklhdflashfalksjdfhlakhsdlfkjhasdfka");
    let session = user_to_session(username, ip1.clone()).unwrap();
    println!("{}", session);
    dbg!(session_to_user(session.clone(), ip1));
    dbg!(session_to_user(session, ip2));
}
