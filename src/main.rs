use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, App, HttpServer};
use env_logger::Env;
use torchguard::{common, db::Db};

mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let port: u16 = std::env::var("PORT")
        .unwrap_or(String::from("8080"))
        .parse()
        .unwrap_or(8080);

    let key_str = std::env::var("AUTH_KEY").expect("No authorization secret key was found.");
    let key = Key::from(key_str.as_bytes());

    let db = Db::from_url(
        std::env::var("DATABASE_URL").expect("No database username was specified."), 
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %r %s %b %{Referer}i %T"))
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                key.clone(),
            ))
            .app_data(db.clone())
            .configure(common::config)
            .configure(user::init)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
