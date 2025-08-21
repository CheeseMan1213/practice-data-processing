use wasm_bindgen::JsCast;
use js_sys::Object;
use web_sys::{js_sys, wasm_bindgen};
// use std::env;

// pub fn load_env() {
//     dotenvy::dotenv().expect("Failed to load .env file.");
// }

// pub fn get_app_host() -> String {
//     env::var("APP_HOST").expect("APP_HOST must be set")
// }

// pub fn get_app_host() -> String {
//     let config = gloo::utils::window().get("__APP_CONFIG__");
//     config.expect("APP_CONFIG not found").unchecked_into::<Object>()
//         .get("APP_HOST")
//         .as_string()
//         .expect("APP_HOST not set")
// }

