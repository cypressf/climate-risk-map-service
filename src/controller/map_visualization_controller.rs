use actix_web::{get, web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}

#[get("/map_visualization/")]
async fn get() -> impl Responder {
    HttpResponse::NotImplemented().finish()
}
