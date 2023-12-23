use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index(path: web::Path<(u32, String)>) -> impl Responder {
    let (user_id, friend) = path.into_inner();
    HttpResponse::Ok().body(format!("Welcome {}, user_id {}!", friend, user_id))
}

// #[get("/u/{user_id}/{age}")]
// async fn u_get(path::Path<(u32, u32)>) -> impl Responder {
//     let (user_id, age) = path.into_inner();

//     HttpResponse::Ok().body(format!("user here: {}, age: {}", user_id, age));
// }

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(index).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
