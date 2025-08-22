use gloo_net::http::Request;
use gloo_net::Error;
use serde_json::json;
use crate::config::config::get_api_host;

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
  let app_host = get_api_host();
  let reponse = Request::post(&format!("{}/users", app_host))
    .json(&json!({
        "email": username,
        "password": password
    }))?
    .send()
    .await?;

  reponse.json::<LoginResponse>().await
}

pub async fn api_me(token: &String) -> Result<MeResponse, Error> {
  let app_host = get_api_host();
  let reponse = Request::get(&format!("{}/me", app_host))
    .header("Authorization", &format!("Bearer {}", token))
    .send()
    .await?;

  reponse.json::<MeResponse>().await
}

pub async fn api_hello() -> Result<String, Error> {
  let app_host = get_api_host();
  let response = Request::get(&format!("{}/hello", app_host))
    .send()
    .await?;

  response.text().await
}