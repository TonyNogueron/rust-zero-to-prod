use actix_web::{dev::Server, get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

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
    })
    .listen(listener)?
    .run();
    Ok(server)
}