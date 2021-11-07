use diesel::r2d2::ConnectionManager;
use diesel::Connection;
use r2d2::{Pool, PooledConnection};

#[derive(Clone)]
pub struct Repo<T>
  where
    T: Connection + 'static,
{
  connection_pool: Pool<ConnectionManager<T>>,
}

impl<T> Repo<T>
  where
    T: Connection + 'static,
{
  pub fn new(database_url: &str) -> Self {
    Self::from_pool_builder(database_url, r2d2::Builder::default())
  }

  pub fn from_pool_builder(database_url: &str, builder: r2d2::Builder<ConnectionManager<T>>) -> Self {
    let manager = ConnectionManager::<T>::new(database_url);
    let connection_pool = builder
      .build(manager)
      .expect("Failed to create pool.");
    Repo { connection_pool }
  }
  // getter
  pub fn conn(&self) -> PooledConnection<ConnectionManager<T>> {
    self.connection_pool.get().unwrap()
  }
}