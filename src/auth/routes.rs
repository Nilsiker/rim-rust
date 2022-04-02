use actix_session::Session;
use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};

#[get("/auth")]
pub async fn get(session: Session) -> impl Responder {
    if let Ok(v) = session.get::<String>("session_id") {
        match v {
            Some(id) => {
                HttpResponse::Ok().body(format!("You're now authorized, Mr. Jon von Bonvi"))
            }
            None => {
                session.insert("session_id", "1").unwrap();
                HttpResponse::Unauthorized()
                    .body("Sorry buddy, you ain't comin in... Unless you refresh!")
            }
        }
    } else {
        HttpResponse::InternalServerError().body("")
    }
}

pub fn init(cfg: &mut ServiceConfig) {
        cfg.service(get);
    }