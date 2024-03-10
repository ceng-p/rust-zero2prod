use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn manual_hello() -> HttpResponse {
    HttpResponse::Ok().body("Hey there!")
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/hello", web::get().to(manual_hello))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
