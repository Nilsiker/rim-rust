use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, App, HttpServer};
use env_logger::Env;
use mysql::{Opts, Pool, PooledConn};
use torchguard::{common};

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

    let db = DbHelper {
        username: std::env::var("DB_USERNAME").expect("No database username was specified."),
        password: std::env::var("DB_PASSWORD").expect("No database password was specified."),
        hostname: std::env::var("DB_HOSTNAME").expect("No database hostname was specified."),
    };


    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%a %r %s %b %{Referer}i %T"))
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                key.clone(),
            ))
            .configure(common::config)
            .configure(user::init)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

struct DbHelper {
    pub username: String,
    pub password: String,
    pub hostname: String,
}

impl DbHelper {
    fn url(self) -> String {
        format!(
            "mysql://{}:{}@{}/torchguard",
            self.username, self.password, self.hostname
        )
    }

    pub fn pool(self) -> Pool {
        Pool::new(Opts::from_url(&self.url()).unwrap()).unwrap()
    }

    pub fn conn(self) -> PooledConn {
        self.pool().get_conn().unwrap()
    }
}
