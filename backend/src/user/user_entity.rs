// This is similar to an Entiry class in Java. It is used to define the structure of the User object.
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct User {
  pub email: String,
  pub password: String,
}