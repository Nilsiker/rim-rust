mod routes;
pub use routes::init;
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Get {
    pub id: Option<Uuid>,
    pub q: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Delete {
    pub id: Option<String>,
}
