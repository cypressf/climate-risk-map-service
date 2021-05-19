use actix_web::{get, web, App, HttpServer};
use std::sync::Mutex;

struct AppState {
    counter: Mutex<i32>,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Hi Cypress, the count is {}", counter)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppState {
        counter: Mutex::new(0),
    });
    HttpServer::new(move || App::new().app_data(counter.clone()).service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
