use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct QueryParams {
    name: Option<String>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("params")]
async fn params(query: web::Query<QueryParams>) -> impl Responder {
    if let Some(value) = &query.name {
        HttpResponse::Ok().body(format!("name: {}", value))
    } else {
        HttpResponse::Ok().body("params not found")
    }
}

async fn mannual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(params)
            .route("/hey", web::get().to(mannual_hello))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
