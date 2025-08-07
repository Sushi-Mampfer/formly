use aes_gcm::aead::Aead;
use base64::{engine::general_purpose::URL_SAFE, Engine as _};

use crate::{CIPHER, NONCE};

pub fn session_to_user(session: String, ip: String) -> Option<String> {
    let ciphertext = URL_SAFE.decode(session).ok()?;
    let decoded = String::from_utf8(CIPHER.decrypt(&NONCE, ciphertext.as_slice()).ok()?).ok()?;
    let splits: Vec<&str> = decoded.split(';').collect();
    match splits.len() {
        2 => {
            if splits[1] == ip {
                Some(splits[0].to_string())
            } else {
                None
            }
        }
        _ => None
    }
}

pub fn user_to_session(username: String, ip: String) -> Option<String> {
    let mut plaintext = String::new();
    plaintext.push_str(&username);
    plaintext.push(';');
    plaintext.push_str(&ip);
    Some(URL_SAFE.encode(CIPHER.encrypt(&NONCE, plaintext.as_bytes()).ok()?))

}