use crate::auth::verify;
use crate::auth::Post;
use crate::entity::users::{Column, Entity as User};
use actix_session::Session;
use actix_web::web::{Data, Json};
use actix_web::{get, post, web::ServiceConfig, HttpResponse, Responder};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use torchguard::common::AppState;

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

#[post("/auth")]
pub async fn post(json: Json<Post>, data: Data<AppState>, session: Session) -> impl Responder {
    let Post { email, password } = json.into_inner();
    let db = &data.pool;
    match User::find().filter(Column::Email.eq(email)).one(db).await {
        Ok(user) => {
            if let Some(user) = user {
                let salt: &[u8] = &user.salt;
                let hash: &[u8] = &user.hash;
                match verify(password, salt, hash) {
                    Ok(_) => match session.insert("user_id", user.id) {
                        Ok(_) => {
                            // TODO give oauth token to client
                            HttpResponse::Ok().body("You're now authorized, Mr. Jon von Bonvi")
                        }
                        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
                    },
                    Err(_) => HttpResponse::Unauthorized().body(format!("Wrong password.")),
                }
            } else {
                HttpResponse::Unauthorized().body("User not found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(get);
    cfg.service(post);
}
