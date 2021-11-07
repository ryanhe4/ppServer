extern crate actix_web;

use actix_web::{App, HttpServer, Scope, ResponseError};
use actix_web::http::Error;
use actix_web::web::Data;
use rand::Rng;
use application::{api, logger};
use database::{Repo};
use database::models::NewRand;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  logger::initialize_logger();
  HttpServer::new(|| {
    App::new()
      .data(Repo::new("mysql://root:postdcpost@127.0.0.1:3307/MYWEB_DB"))
      .wrap(logger::Logger::default())
      .wrap(logger::Logger::new("%a %{User-Agent}i"))
      .configure(api::routes)
  })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}