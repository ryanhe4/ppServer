extern crate actix_web;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Scope};
use application::logger;

async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello PostPost Server")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
  HttpResponse::Ok().body("Login")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  logger::initialize_logger();

  HttpServer::new(|| {
    App::new()
      .wrap(logger::Logger::default())
      .wrap(logger::Logger::new("%a %{User-Agent}i"))
      .configure(routes)
  })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn routes(app: &mut web::ServiceConfig) {
  app
    .service(web::resource("/").to(index))
    .service(web::scope("/api")
      .configure(user_route));
}

fn user_route(user: &mut web::ServiceConfig) {
  user.service(web::scope("/users")
    .service(web::scope("/login")
      .route("", web::get().to(manual_hello))));
}