use actix_web::{
        delete, get, post,
        web::{Data, Json, Query, ServiceConfig},
        HttpResponse, Responder,
    };
    use mysql::Pool;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Get {
        pub username: Option<String>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Post {
        pub username: String,
        pub email: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Delete {
        pub id: Option<String>,
    }



    #[get("/users")]
    pub async fn get(query: Query<Get>, pool: Data<Pool>) -> impl Responder {
        match pool.get_conn() {
            Ok(conn) => {
                if let Some(username) = &query.username {
                } else {
                    
                }
                HttpResponse::Ok().body("TODO")
            }
            Err(e) => HttpResponse::ServiceUnavailable().body("Could not connect to database."),
        }
    }

    #[post("/users")]
    pub async fn post(body: Json<Post>) -> impl Responder {
        let username = &body.username;
        let email = &body.email;

        HttpResponse::Ok().body(format!(
            "TODO succeed if user is added\n{username}\n{email}"
        ))
    }

    #[delete("/users")]
    pub async fn delete(query: Query<Delete>) -> impl Responder {
        let key_str = std::env::var("ADMIN_KEY").unwrap();
        if let Some(username) = &query.id {
            HttpResponse::Ok().body(format!("TODO get user with username: {username}"))
        } else {
            HttpResponse::Ok().body(format!("TODO gets all users"))
        }
    }

    pub fn init(cfg: &mut ServiceConfig) {
        cfg.service(get);
        cfg.service(post);
        cfg.service(delete);
    }