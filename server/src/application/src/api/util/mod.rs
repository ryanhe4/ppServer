use actix_web::{Error, HttpResponse, ResponseError, web};
use actix_web::web::Data;
use database::models::NewRand;
use database::Repo;

async fn type1(pool: Data<Repo>) -> Result<HttpResponse, Error> {
  use rand::Rng;
  let mut rng = rand::thread_rng();

  let new_data = NewRand {
    value1: rng.gen_range(1..100),
    value2: rng.gen_range(1..100),
    value3: rng.gen_range(1..100),
    value4: rng.gen_range(1..100),
    value5: rng.gen_range(1..100),
    value6: rng.gen_range(1..100),
  };
  let result = web::block(move || {
    database::queries::util_rand::insert(&pool, new_data)
  })
    .await
    .map_err(|e| {
      eprintln!("{}", e.status_code());
      HttpResponse::InternalServerError().body("Internal Server Error")
    }).unwrap();
  Ok(HttpResponse::Ok().json(result))
}

pub fn util_routes(user: &mut web::ServiceConfig) {
  user.service(web::scope("/utils")
    .service(web::scope("/rand_type1")
      .route("", web::get().to(type1))));
}