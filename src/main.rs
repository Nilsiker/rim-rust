use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, SessionMiddleware, SessionMiddlewareBuilder};
use actix_web::{cookie::{Key, SameSite}, middleware::Logger, web, App, HttpServer};

use db::build_db_pool;
use env_logger::Env;

use torchguard::common::{self, AppState};
mod auth;
mod db;
mod entity;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let key_str = std::env::var("AUTH.COOKIE_KEY").expect("AUTH.COOKIE_KEY must be set");
    let key = Key::from(key_str.as_bytes());
    let port = std::env::var("PORT")
        .expect("No port was found in env.")
        .parse::<u16>()
        .expect("Port was not a number.");
    let hostname = std::env::var("HOSTNAME").expect("No hostname was found in env.");

    let state = AppState {
        pool: build_db_pool().await,
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %r %s %b %{Referer}i %T"))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                    .cookie_name("torchguard".to_string())
                    .cookie_secure(false)
                    .cookie_http_only(false)
                    .cookie_domain(Some("localhost".to_string()))
                    .build(),
            )
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allow_any_method()
                    .allow_any_header()
                    .send_wildcard()
                    .supports_credentials(),
            )
            .app_data(web::Data::new(state.clone()))
            .configure(common::init)
            .configure(user::init)
            .configure(auth::init)
    })
    .bind((hostname, port))?
    .run()
    .await
}
