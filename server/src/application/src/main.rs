extern crate actix_web;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Scope};
use actix_web::http::Error;
use actix_web::web::Data;
use application::logger;
use database::{Repo};
use database::models::NewRand;

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

async fn pool_test(pool: Data<Repo>) -> impl Responder {
  let newData = NewRand {
    id: 1,
    value1: 1,
    value2: 1,
    value3: 10,
    value4: 1,
    value5: 1,
    value6: 1,
  };
  let result = web::block(move || {
    database::queries::util_rand::insert(pool.get_ref(), newData)
  })
    .await;
  HttpResponse::Ok().body("Hello PostPost Server")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  logger::initialize_logger();
  HttpServer::new(|| {
    App::new()
      .data(Repo::new("mysql://root:postdcpost@127.0.0.1:3307/MYWEB_DB"))
      // .wrap(logger::Logger::default())
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
      .route("", web::get().to(pool_test))));
}