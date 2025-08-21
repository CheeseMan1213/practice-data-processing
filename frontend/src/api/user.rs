use gloo_net::http::Request;
use gloo_net::Error;
use serde_json::json;
// use crate::config::config::{get_app_host, load_env};

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
  // load_env();
  // let app_host = get_app_host();
  // let app_host = "http://127.0.0.1:3002";
  let app_host = "http://pdp-app-backend:3002";
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
  // load_env();
  // let app_host = get_app_host();
  // let app_host = "http://127.0.0.1:3002";
  let app_host = "http://pdp-app-backend:3002";
  let reponse = Request::get(&format!("{}/me", app_host))
    .header("Authorization", &format!("Bearer {}", token))
    .send()
    .await?;

  reponse.json::<MeResponse>().await
}

pub async fn api_hello() -> Result<String, Error> {
  // load_env();
  // let app_host = get_app_host();
  // let app_host = "http://127.0.0.1:3002";
  let app_host = "http://pdp-app-backend:3002";
  let response = Request::get(&format!("{}/hello", app_host))
    .send()
    .await?;

  response.text().await
}