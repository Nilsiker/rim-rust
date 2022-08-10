
pub struct SessionKeys {
    pub user_id: &'static str
}

pub static SESSION_KEYS: SessionKeys = SessionKeys {
    user_id: "user_id",
};

pub mod common {
    use actix_session::Session;
    use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};
    use sea_orm::DatabaseConnection;

    use crate::SESSION_KEYS;

    #[derive(Clone)]
    pub struct AppState {
        pub pool: DatabaseConnection,
    }

    #[get("/")]
    pub async fn index(session: Session) -> impl Responder {
        let mut index = String::from("<b>Torchguard v0.1</b><br/>A backend server for a Burning Wheel companion web app.<br/><br/>");
        match session.get::<String>(SESSION_KEYS.user_id) {
            Ok(user_id) => {
                if let Some(user_id) = user_id {
                    index += &format!("You are logged in as user {}.", user_id);
                } else {
                    index += "You are not logged in.<br><button>Login</button>";
                }
            },
            Err(_) => {
                index += "You are not logged in. An error occured...";
            }            
        }

        HttpResponse::Ok().body(index)
    }

    pub fn init(cfg: &mut ServiceConfig) {
        cfg.service(index);
    }
}
