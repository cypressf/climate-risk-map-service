use super::AppState;
use actix_web::{get, web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}

#[get("/data/")]
async fn get(app_state: web::Data<AppState<'_>>) -> impl Responder {
    println!("GET: /data/");

    let states = app_state.database.state.all().await;

    match states {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(states) => HttpResponse::Ok().json(states),
    }
}
