use crate::auth::add_user_id_to_session_cookie;
use crate::auth::verify;
use crate::auth::Post;
use crate::entity::users::{Column, Entity as User};
use actix_session::Session;
use actix_web::web::{Data, Json};
use actix_web::{post, web::ServiceConfig, HttpResponse, Responder};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use torchguard::common::AppState;

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
                    Ok(_) => add_user_id_to_session_cookie(session, user.id),
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
    cfg.service(post);
}
