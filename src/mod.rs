
    use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};

    #[get("/")]
    pub async fn index() -> impl Responder {
        HttpResponse::Ok().body("TODO api info goes here, someday")
    }

    pub fn config(cfg: &mut ServiceConfig) {
        cfg.service(index);
    }
