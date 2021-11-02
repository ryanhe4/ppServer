use crate::models::{NewRand, UtilRand};
use crate::queries::{util_rand};
use crate::Repo;
use diesel::result::{DatabaseErrorKind, Error};

pub struct Repository(pub Repo);