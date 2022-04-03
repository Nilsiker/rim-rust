pub mod db {
    use diesel::{
        r2d2::{self, ConnectionManager, PooledConnection},
        PgConnection,
    };

    type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
    pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

    #[derive(Clone)]
    pub struct Db {
        pool: Pool,
    }

    impl Db {
        pub fn from_url(db_url: String) -> Self {
            let manager = ConnectionManager::<PgConnection>::new(db_url);
            let pool = Pool::new(manager).expect("Failed to create database pool");
            Db { pool }
        }

        pub fn connection(&self) -> Result<DbConnection, r2d2::PoolError> {
            self.pool.get()
        }
    }
}

pub mod common {
    use actix_session::Session;
    use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};

    #[get("/")]
    pub async fn index(session: Session) -> impl Responder {
        if let Ok(key) = std::env::var("KEY") {
            print!("{key}");
            match session.get::<String>("session_id") {
                Ok(_) => (),
                Err(_) => {
                    session.insert("session_id", "1").unwrap();
                    ()
                }
            };
            let entries = session.entries();
            HttpResponse::Ok().body(format!(
                "TODO api info goes here, someday. You have session: {:?}",
                entries
            ))
        } else {
            HttpResponse::InternalServerError().body("")
        }
    }

    pub fn config(cfg: &mut ServiceConfig) {
        cfg.service(index);
    }
}
