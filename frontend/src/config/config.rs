use dotenvy::dotenv;
use std::env;

pub fn load_env() {
    dotenv().ok();
}

pub fn get_app_host() -> String {
    env::var("APP_HOST").expect("APP_HOST must be set")
}
