use actix_web::http::header;
use actix_web::middleware::cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, Responder};

fn greet(info: web::Path<String>) -> impl Responder {
    format!("Hello {}!", info)
}

fn main() -> std::io::Result<()> {
    println!("running application server on port 8080");
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:8001")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(web::resource("/{name}").to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
}
