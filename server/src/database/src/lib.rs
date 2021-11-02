#[macro_use]
extern crate diesel;

pub mod schema;
pub mod connection;
pub mod models;
pub mod queries;
pub mod repository;

use diesel::MysqlConnection;

pub type Repo = connection::Repo<MysqlConnection>;