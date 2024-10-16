use axum::extract::State;
use axum::http::StatusCode;
use serde_json::json;
use sqlx::PgPool;

use crate::user::user_entity::User;

pub async fn get_all_users(
    State(db_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let rows = sqlx::query_as!(User, "SELECT * FROM users ORDER BY email")
        .fetch_all(&db_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "message": e.to_string()}).to_string(),
            )
        })?;

    Ok((
        StatusCode::OK,
        json!({"success": true, "data": rows}).to_string(),
    ))
}

// pub async fn get_user_by_email(
//   State(db_pool): State<PgPool>,
//   id: i32,
// ) -> Result<(StatusCode, String), (StatusCode, String)> {
//   let row = sqlx::query_as!(user_entity, "SELECT * FROM users WHERE email = $1", email)
//     .fetch_one(&db_pool)
//     .await
//     .map_err(|e| {
//       (
//         StatusCode::INTERNAL_SERVER_ERROR,
//         json!({"success": false, "message": e.to_string()}).to_string(),
//       )
//     })?;

//   Ok((
//     StatusCode::OK,
//     json!({"success": true, "data": row}).to_string(),
//   ))
// }