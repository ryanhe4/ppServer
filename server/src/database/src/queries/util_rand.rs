use diesel::result::Error;
use diesel::RunQueryDsl;
use crate::models::{NewRand, UtilRand};
use crate::schema::util_rand as util_rand_table;
use crate::Repo;

pub fn insert(repo: &Repo, util_rand: NewRand) -> Result<UtilRand, Error> {
  diesel::insert_into(util_rand_table::table)
    .values(&util_rand)
    .get_result(&repo.conn())
}