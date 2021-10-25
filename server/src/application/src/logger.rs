pub use actix_web::middleware::Logger;
pub use env_logger::Env;

pub fn initialize_logger() {
  env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
  println!("Init Logger!!");
}