use crate::auth::{salt, uuid, hash};
use crate::entity::users::{self, Entity as User};
use crate::{
    entity::users::Column,
    user::{Delete, Get, Post},
};
use actix_web::{
    body::BoxBody,
    delete, get, post,
    web::{Data, Json, Query, ServiceConfig},
    HttpResponse, Responder,
};

use sea_orm::prelude::Uuid;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter,
    QuerySelect, Set,
};
use torchguard::common::AppState;
#[get("/users")]
pub async fn get(query: Query<Get>, data: Data<AppState>) -> HttpResponse<BoxBody> {
    let conn = &data.pool;
    let Query(Get { id, q }) = query;

    match id {
        Some(id) => find_by_id(id, conn).await,
        None => match q {
            Some(filter) => find_with_filter(filter, conn).await,
            None => find_all(conn).await,
        },
    }
}
#[post("/users")]
pub async fn post(body: Json<Post>, data: Data<AppState>) -> impl Responder {
    let Post { email, password } = body.into_inner();
    let uuid = uuid();
    let salt = match salt() {
        Ok(salt) => salt,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{e}"));
        }
    };
    let hash = hash(&password, &salt);

    let model = users::ActiveModel {
        id: Set(uuid),
        email: Set(email),
        hash: Set(hash.into()),
        salt: Set(salt.into()),
    };

    let db = &data.pool;
    match model.insert(db).await {
        Ok(inserted_model) => HttpResponse::Ok().body(format!("{}", inserted_model.id)),
        Err(e) => HttpResponse::InternalServerError().body(format!("{e}")),
    }
}
#[delete("/users")]
pub async fn delete(query: Query<Delete>) -> impl Responder {
    if let Some(username) = &query.id {
        HttpResponse::Ok().body(format!("TODO get user with username: {username}"))
    } else {
        HttpResponse::Ok().body(format!("TODO gets all users"))
    }
}

async fn find_all(conn: &DatabaseConnection) -> HttpResponse<BoxBody> {
    match User::find()
        .select_only()
        .column(Column::Id)
        .column(Column::Email)
        .into_json()
        .all(conn)
        .await
    {
        Ok(users) => match users.len() {
            0 => HttpResponse::NotFound().body("No users found."),
            _ => HttpResponse::Ok().json(users),
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn find_with_filter(filter: String, conn: &DatabaseConnection) -> HttpResponse<BoxBody> {
    match User::find()
        .select_only()
        .column(Column::Id)
        .column(Column::Email)
        .filter(
            Condition::any()
                .add(Column::Email.contains(&filter)),
        )
        .into_json()
        .all(conn)
        .await
    {
        Ok(users) => match users.len() {
            0 => HttpResponse::NotFound().body("No users found."),
            _ => HttpResponse::Ok().json(users),
        },
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn find_by_id(id: Uuid, conn: &DatabaseConnection) -> HttpResponse<BoxBody> {
    match User::find_by_id(id)
        .select_only()
        .column(Column::Id)
        .column(Column::Email)
        .into_json()
        .one(conn)
        .await
    {
        Ok(v) => {
            if let Some(user) = v {
                HttpResponse::Ok().json(user)
            } else {
                HttpResponse::NotFound().body("User not found.")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub fn init(cfg: &mut ServiceConfig) {
    cfg.service(get);
    cfg.service(post);
    cfg.service(delete);
}
