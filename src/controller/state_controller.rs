use super::AppState;
use actix_web::{get, web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}

#[get("/state/{id}")]
async fn get(id: web::Path<String>, app_state: web::Data<AppState<'_>>) -> impl Responder {
    println!("GET: /state/{}", id);

    let state = app_state.database.state.by_id(&id).await;

    match state {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(state) => HttpResponse::Ok().json(state),
    }
}
