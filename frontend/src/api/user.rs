use gloo_net::http::Request;
use gloo_net::Error;
use serde_json::json;

use super::APP_HOST;
#[derive(PartialEq)]
pub struct User {
  pub id: i32,
  pub username: String,
  pub created_at: String,
}

#[derive(serde::Deserialize)]
pub struct LoginResponse {
  pub token: String,
}

#[derive(serde::Deserialize)]
pub struct MeResponse {
  pub id: i32,
  pub username: String,
  pub created_at: String,
}

pub async fn api_login(username: String, password: String) -> Result<LoginResponse, Error> {
  let reponse = Request::post(&format!("{}/users", APP_HOST))
    .json(&json!({
        "email": username,
        "password": password
    }))?
    .send()
    .await?;

  reponse.json::<LoginResponse>().await
}

pub async fn api_me(token: &String) -> Result<MeResponse, Error> {
  let reponse = Request::get(&format!("{}/me", APP_HOST))
    .header("Authorization", &format!("Bearer {}", token))
    .send()
    .await?;

  reponse.json::<MeResponse>().await
}

pub async fn api_hello() -> Result<String, Error> {
  let response = Request::get(&format!("{}/hello", APP_HOST))
    .send()
    .await?;

  response.text().await
}