use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use auth::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/loggin")]
async fn loggin(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/loggout")]
async fn loggout(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/register")]
async fn register(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/unsubscribe")]
async fn unsubscribe(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(loggin)
            .service(loggout)
            .service(register)
            .service(unsubscribe)
    })
    .bind(("127.0.0.1", 7777))?
    .run()
    .await
}
