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
