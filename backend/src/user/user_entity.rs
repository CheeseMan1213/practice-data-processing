// This is similar to an Entiry class in Java. It is used to define the structure of the User object.
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// This is the actual entity
#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
  pub email: String,
  pub password: String,
}

// This is a different struct that I needed for some reasonto get the update user crud operation to worlk
#[derive(Deserialize)]
pub struct UpdateUserRequest {
  pub email: String,
  pub password: String,
}