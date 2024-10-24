use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::json;
use serde_json::Value;
use sqlx::PgPool;

use crate::user::user_entity::UpdateUserRequest;
use crate::user::user_entity::User;

pub async fn hello(
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    Ok((
        StatusCode::OK,
        Json(json!({"success": true, "message": "Hello, World! backend James is the coolest."})),
    ))
}

pub async fn get_all_users(
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let rows = sqlx::query_as!(User, "SELECT * FROM users ORDER BY email")
        .fetch_all(&db_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"success": false, "message": e.to_string()})),
            )
        })?;

    Ok((
        StatusCode::OK,
        Json(json!({"success": true, "data": rows})),
    ))
}

pub async fn get_user_by_email(
  State(db_pool): State<PgPool>,
  Path(email): Path<String>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
  let row = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
    .fetch_one(&db_pool)
    .await
    .map_err(|e| {
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"success": false, "message": e.to_string()})),
      )
    })?;

  Ok((
    StatusCode::OK,
    Json(json!({"success": true, "data": row})),
  ))
}

pub async fn create_user(
    State(db_pool): State<PgPool>,
    Json(user): Json<User>,
  ) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    let row = sqlx::query_as!(
        User,
      "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING email, password",
      user.email,
      user.password
    )
    .fetch_one(&db_pool)
    .await
    .map_err(|e| {
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"success": false, "message": e.to_string()})),
      )
    })?;
  
    Ok((
      StatusCode::CREATED,
      Json(json!({"success": true, "data": row})),
    ))
  }

pub async fn update_user(
  State(db_pool): State<PgPool>,
  Json(update_user_request): Json<UpdateUserRequest>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
  sqlx::query!(
    "UPDATE users SET password = $2 WHERE email = $1",
    update_user_request.email,
    update_user_request.password
  )
  .execute(&db_pool)
  .await
  .map_err(|e| {
    (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(json!({"success": false, "message": e.to_string()})),
    )
  })?;

  Ok((
    StatusCode::OK,
    Json(json!({"success": true}))))
}

pub async fn delete_user(
  State(db_pool): State<PgPool>,
  Json(user): Json<User>,
) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
  sqlx::query!("DELETE FROM users WHERE email = $1", user.email,)
    .execute(&db_pool)
    .await
    .map_err(|e| {
      (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"success": false, "message": e.to_string()})),
      )
    })?;

    Ok((
      StatusCode::OK,
      Json(json!({"success": true}))))
}
