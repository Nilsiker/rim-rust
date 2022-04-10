pub mod common {
    use actix_session::Session;
    use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};
    use sea_orm::DatabaseConnection;

    #[derive(Clone)]
    pub struct AppState {
        pub pool: DatabaseConnection,
    }

    #[get("/")]
    pub async fn index(session: Session) -> impl Responder {
        match session.get::<u16>("id") {
            Ok(o) => match o {
                Some(_) => HttpResponse::Ok().body(format!(
                    "TODO api info goes here, someday. You have session: {:?}",
                    session.entries()
                )),
                None => {
                    session
                        .insert("id", 1)
                        .expect("Could not insert session id.");
                    HttpResponse::Unauthorized().body("Nu-uh mister! But try again...")
                }
            },
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    }

    pub fn init(cfg: &mut ServiceConfig) {
        cfg.service(index);
    }
}
