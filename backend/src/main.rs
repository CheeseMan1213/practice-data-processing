mod quadratic_formula;
mod math;
mod user;

use std::env;
use axum::http::Method;
use user::user_repository::get_all_users;
use user::user_repository::get_user_by_email;
use user::user_repository::create_user;
use user::user_repository::hello;
use user::user_repository::update_user;
use user::user_repository::delete_user;
use math::add::add;
use math::multiply::multiply;
use quadratic_formula::quadratic_formula::QuadraticFormula;
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::cors::{CorsLayer, Any};

#[cfg(debug_assertions)]
fn load_env() {
    dotenvy::dotenv().expect("Failed to load .env file.");
}

#[cfg(not(debug_assertions))]
fn load_env() {
    // No-op for production/container environment.
    // Get values straight from environment variables.
    let _ = std::env::var("SERVER_PORT").expect("SERVER_PORT is not set in envionment variables.");
    let _ = std::env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS is not set in envionment variables.");
    let _ = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in envionment variables.");
    
}

#[tokio::main]
async fn main() {
    println!("Hello, world from backend.");
    load_env();

    let server_port = env::var("SERVER_PORT").expect("SERVER_PORT is not set in envionment variables or .env file.");
    let server_address = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS is not set in envionment variables or .env file.");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in envionment variables or .env file.");

    let db_pool = PgPoolOptions::new()
    .max_connections(16)
    .connect(&database_url)
    .await
    .expect("Failed to connect to database.");

    println!("{}", add(2, 3));
    
    let qf = QuadraticFormula::new(1.0, -3.0, 2.0);
    let ans = qf.calculate();
    println!("{:?}", ans);
    
    println!("{}", multiply(2, 3));

    // http://127.0.0.1:3002
    let listener = TcpListener::bind(format!("{}:{}", server_address, server_port))
    .await
    .expect("Failed to create TCP listener.");
    
    axum::serve(listener, create_router(db_pool)).await.unwrap();
}

async fn hello_world() -> String {
    "Hello world_backend".to_owned()
}

pub fn create_router(db_pool: Pool<Postgres>) -> Router {

    // Create a CORS layer
    let cors = CorsLayer::new()
    .allow_origin(Any) // Allow any origin
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]) // Allow specific methods
    .allow_headers(Any); // Allow any headers

    Router::new()
        .route("/", get(hello_world))
        .route("/hello", get(hello))
        .route("/users", get(get_all_users).post(create_user))
        .route("/users/:email", get(get_user_by_email).put(update_user).delete(delete_user))
        .layer(cors)
        .with_state(db_pool)
}

#[cfg(test)]
mod test;
