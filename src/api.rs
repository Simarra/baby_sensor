
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::db;

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

fn index() -> impl Responder {
    let statements = db::get_statements();
    HttpResponse::Ok().json(statements)
}

// pub fn create_post(t: &str, b: &str) -> String {
//     let connection = establish_connection();

//     let uuid = Uuid::new_v4().to_hyphenated().to_string();

//     let new_post = NewPost { id: &uuid,  title: t, body: b };

//     diesel::insert_into(posts::table)
//         .values(&new_post)
//         .execute(&connection)
//         .expect("Error saving new post");

//     uuid
// }

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}