use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, web, App, HttpServer};
use auth::oidc_client;
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

    let port = std::env::var("PORT")
        .expect("No port was found in env.")
        .parse::<u16>()
        .expect("Port was not a number.");
    let hostname = std::env::var("HOSTNAME")
        .expect("No hostname was found in env.");

    let state = AppState {
        pool: build_db_pool().await,
        oidc: oidc_client().await,
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %r %s %b %{Referer}i %T"))
            .app_data(web::Data::new(state.clone()))
            .configure(common::init)
            .configure(user::init)
            .configure(auth::init)
    })
    .bind((hostname, port))?
    .run()
    .await
}
