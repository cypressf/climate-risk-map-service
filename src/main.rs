use actix_web::{get, web, App, HttpServer};
use climate_risk_map::config::Config;
use std::sync::Mutex;

struct AppState {
    counter: Mutex<i32>,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Hi the counter is {}", counter)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env();
    let counter = web::Data::new(AppState {
        counter: Mutex::new(0),
    });
    let app = HttpServer::new(move || App::new().app_data(counter.clone()).service(index))
        .bind(config.get_app_url())?;
    println!("Listening on: {}", config.get_app_url());
    app.run().await
}
