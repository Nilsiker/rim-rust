mod routes;
use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    reqwest::{http_client, async_http_client},
    ClientId, ClientSecret, IssuerUrl, RedirectUrl,
};
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
    let n_iter = NonZeroU32::new(100_000).unwrap();

    pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        salt,
        password.as_bytes(),
        hash,
    )
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub email: String,
    pub password: String,
}

pub async fn oidc_client() -> CoreClient {
    let issuer_url = IssuerUrl::new(std::env::var("AUTH.ISSUER_URL").expect("No issuer URL was found."))
        .expect("Could not parse issuer URL.");

    let client_id =
        ClientId::new(std::env::var("AUTH.CLIENT_ID").expect("No client ID was found."));

    let client_secret = ClientSecret::new(
        std::env::var("AUTH.CLIENT_SECRET").expect("No client secret was found."),
    );

    let provider_metadata = CoreProviderMetadata::discover_async(issuer_url, async_http_client).await
        .expect("Could not discover provider metadata.");

    let redirect_url =
        dbg!(RedirectUrl::new(std::env::var("AUTH.REDIRECT_URL").expect("No redirect URL was found."))
            .expect("Could not parse redirect URL."));

    CoreClient::from_provider_metadata(provider_metadata, client_id, Some(client_secret))
        .set_redirect_uri(redirect_url)
}
