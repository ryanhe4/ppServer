use chrono::{NaiveDateTime};
use crate::schema::util_rand;
use diesel::{AsChangeset, Insertable, Queryable};
use diesel::sql_types::Integer;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UtilRand {
  pub id: i32,
  pub value1: i32,
  pub value2: i32,
  pub value3: i32,
  pub value4: i32,
  pub value5: i32,
  pub value6: i32,
  pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug, Clone)]
#[table_name = "util_rand"]
pub struct NewRand {
  pub id: i32,
  pub value1: i32,
  pub value2: i32,
  pub value3: i32,
  pub value4: i32,
  pub value5: i32,
  pub value6: i32,
}
