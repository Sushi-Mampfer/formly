use std::time::{SystemTime, UNIX_EPOCH};

use aes_gcm::aead::Aead;
use axum::http::HeaderMap;
use base64::{Engine as _, engine::general_purpose::URL_SAFE};

use crate::{CIPHER, NONCE};

pub fn headers_to_user(headers: HeaderMap) -> Option<String> {
    return Some("sushi".to_string());
    let ip = match headers
        .get("X-Forwarded-For")
        .expect("X-Forwarded-For header not found.")
        .to_str()
    {
        Ok(ip) => ip.to_string(),
        _ => return None,
    };

    if let Some(h) = headers.get("Cookie") {
        for i in h.to_str().ok()?.split("; ") {
            let cookie: Vec<&str> = i.split("=").collect();
            match cookie.len() {
                2 => {
                    if cookie[0] == "session" {
                        return session_to_user(String::from(cookie[1]), ip);
                    }
                }
                _ => return None,
            }
        }
    }
    None
}

pub fn session_to_user(session: String, ip: String) -> Option<String> {
    let ciphertext = URL_SAFE.decode(session).ok()?;
    let decoded = String::from_utf8(CIPHER.decrypt(&NONCE, ciphertext.as_slice()).ok()?).ok()?;
    let splits: Vec<&str> = decoded.split(';').collect();
    match splits.len() {
        3 => {
            if splits[1] == ip {
                if let Ok(t) = splits[2].parse::<u64>() {
                    if t > SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                    {
                        Some(splits[0].to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn user_to_session(username: String, ip: String) -> Option<String> {
    let mut plaintext = String::new();
    plaintext.push_str(&username);
    plaintext.push(';');
    plaintext.push_str(&ip);
    plaintext.push(';');
    plaintext.push_str(
        &(SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 60 * 60 * 24 * 7)
            .to_string(),
    );
    Some(URL_SAFE.encode(CIPHER.encrypt(&NONCE, plaintext.as_bytes()).ok()?))
}
