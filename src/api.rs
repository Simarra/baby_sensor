
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::db;
use crate::functions;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Deserialize)]
pub struct StatementQueryNumbers {
    nb: i64,
}

#[get("/statements")]
pub async fn get_statements(nb: web::Query<StatementQueryNumbers>) -> impl Responder {
    let statements = db::get_statements(nb.nb);
    HttpResponse::Ok().json(statements)
}

#[derive(Deserialize)]
pub struct StatementQuery {
    temperature: f32,
}

#[post("/statement")]
pub async fn add_statement(temp_infos: web::Query<StatementQuery>) -> impl Responder {
    let uuid_res = db::create_statement(&temp_infos.temperature);
    HttpResponse::Ok().body(uuid_res)
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
            .service(hello)
            .service(echo)
            .service(get_statements)
            .service(add_statement)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("localhost:8080")?
    .run()
    .await
}