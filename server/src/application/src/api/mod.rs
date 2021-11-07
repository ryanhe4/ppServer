pub mod user;
pub mod util;

use actix_web::{HttpResponse, Responder, web};

async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello PostPost Server")
}

pub fn routes(app: &mut web::ServiceConfig) {
  app
    .service(web::resource("/").to(index))
    .service(web::scope("/api")
      .configure(user::user_routes)
      .configure(util::util_routes));
}