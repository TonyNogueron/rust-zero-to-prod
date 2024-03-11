use actix_web::{
    dev::Server, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use std::net::TcpListener;

#[derive(Debug, Deserialize, Serialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[post("/subscriptions")]
async fn json_handler(data: web::Json<FormData>) -> impl Responder {
    println!("Received JSON data: {:?}", data);
    format!("Received JSON data: {:?}", data)
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(health_check)
            .route("/greet/{name}", web::get().to(greet))
            .service(json_handler)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
