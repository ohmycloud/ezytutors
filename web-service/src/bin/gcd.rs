use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Header<T>(pub T);

#[derive(Deserialize)]
struct Info {
    username: String,
}

// deserialize `Info` from request's body
#[post("/submit")]
async fn submit(info: web::Json<Info>) -> String {
    format!("Welcome {}!", info.username)
}

// extract path info using serde
#[get("/users/{username}")]
async fn user(info: web::Path<Info>) -> String {
    format!("Welcome {}!", info.username)
}

// data is passed in here through query parameters in the URL
// for example, google.com/?hello=world
#[get("/")]
async fn home(info: web::Query<Info>) -> String {
    format!("Welcome {}!", info.username)
}

#[post("/")]
async fn form(form: web::Form<Info>) -> actix_web::Result<String> {
    Ok(format!("Welcome {}!", form.username))
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok()))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{}", "Listenting on 127.0.0.1:8080");
    HttpServer::new(|| {
        // App::new()
        //     .service(index)
        //     .service(echo)
        //     .route("/hey", web::get().to(manual_hello))
        App::new().configure(config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
