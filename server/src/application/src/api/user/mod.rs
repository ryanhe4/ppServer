use actix_web::{HttpResponse, Responder, web};

async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello PostPost users/login")
}

pub fn user_routes(user: &mut web::ServiceConfig) {
  user.service(web::scope("/users")
    .service(web::scope("/login")
      .route("", web::get().to(index))));
}