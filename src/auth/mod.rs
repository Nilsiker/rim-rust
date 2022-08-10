mod routes;
use actix_session::Session;
use actix_web::{HttpResponse};
pub use routes::init;

use ring::{digest, pbkdf2, rand::SecureRandom};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

pub fn uuid() -> Uuid {
    Uuid::new_v4()
}

pub fn salt() -> Result<[u8; digest::SHA512_OUTPUT_LEN], ring::error::Unspecified> {
    let rng = ring::rand::SystemRandom::new();
    let mut salt = [0u8; digest::SHA512_OUTPUT_LEN];
    match rng.fill(&mut salt) {
        Ok(_) => Ok(salt),
        Err(e) => Err(e),
    }
}

pub fn hash(password: &str, salt: &[u8]) -> [u8; digest::SHA512_OUTPUT_LEN] {
    let mut hash = [0u8; digest::SHA512_OUTPUT_LEN];
    let n_iter = NonZeroU32::new(100_000).unwrap();

    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut hash,
    );

    hash
}

pub fn verify(password: String, salt: &[u8], hash: &[u8]) -> Result<(), ring::error::Unspecified> {
    let n_iter = std::env::var("AUTH.ITERATIONS")
        .expect("AUTH.ITERATIONS must be set")
        .parse::<NonZeroU32>()
        .expect("AUTH.ITERATIONS must be a non-zero positive integer");

    pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        salt,
        password.as_bytes(),
        hash,
    )
}

pub fn add_user_id_to_session_cookie(session: Session, id: Uuid) -> HttpResponse {
    match session.insert("user_id", id.to_string()) {
        Ok(_) => HttpResponse::Ok().body("Authenticated!"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn is_authenticated(session:Session) -> bool {
    match session.get::<String>("user_id") {
        Ok(o) => match o {
            Some(_) => true,    // TODO actually check db if user exists (easily spoofed now)
            None => false,
        },
        Err(_) => false,
    }
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub email: String,
    pub password: String,
}
