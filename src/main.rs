mod model;
mod response;

use actix_web::{App, get, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use crate::response::GenericResponse;


#[get("api/healthcheck")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build simple rest CRUD api with actix";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };

    HttpResponse::Ok().json(response_json)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        App::new()
            .service(health_checker_handler)
            .wrap(Logger::default())
    })
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
