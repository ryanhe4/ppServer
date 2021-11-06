use diesel::result::Error;
use diesel::prelude::*;
use crate::models::{NewRand, UtilRand};
use crate::schema::util_rand;
use crate::Repo;

pub fn insert(repo: &Repo, new_util_rand: NewRand) -> Result<UtilRand, Error> {
  use util_rand::dsl::{util_rand as all_util_rand, id};

  diesel::insert_into(all_util_rand)
    .values(&new_util_rand)
    .execute(&repo.conn())
    .expect("Error While Saving Rand_Values");
  all_util_rand.order(id.desc()).first(&repo.conn())
}