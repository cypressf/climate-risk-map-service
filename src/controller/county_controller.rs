use super::log_request;
use super::AppState;
use actix_web::{get, web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}

#[get("/county/{id}")]
async fn get(id: web::Path<String>, app_state: web::Data<AppState<'_>>) -> impl Responder {
    log_request("GET: /county", &app_state.connections);

    let county = app_state.database.county.by_id(&id).await;

    match county {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(county) => HttpResponse::Ok().json(county),
    }
}
