mod quadratic_formula;
mod math;
use math::add::add;
use math::multiply::multiply;
use quadratic_formula::quadratic_formula::QuadraticFormula;
use axum::{routing::get, Router};
use tokio::net::TcpListener;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

// pub static PORT: &'static str = "3002";

#[tokio::main]
async fn main() {
    println!("Hello, world from backend.");
    dotenvy::dotenv().expect("Failed to load .env file.");

    let server_port = std::env::var("SERVER_PORT").expect("SERVER_PORT is not set in .env file.");
    let server_address = std::env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS is not set in .env file.");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file.");

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

    // let app = Router::new().route("/", get(|| async { "Hello, World! backend James is the coolest." }));

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // http://127.0.0.1:3002
    let listener = TcpListener::bind(format!("{}:{}", server_address, server_port))
    .await
    .expect("Failed to create TCP listener.");
    // axum::serve(listener, app).await.unwrap();
    axum::serve(listener, create_router(db_pool)).await.unwrap();
}

async fn hello_world() -> String {
    "Hello world_00000".to_owned()
}

pub fn create_router(db_pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", get(hello_world))
        // .route("createQuadraticFormula", get(create_quadratic_formula))
        // .route("getOneQuadraticFormula", get(get_one_quadratic_formula))
        .with_state(db_pool)
}


#[cfg(test)]
mod test;
