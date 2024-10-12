mod quadratic_formula;
mod math;
use math::add::add;
use math::multiply::multiply;
use quadratic_formula::quadratic_formula::QuadraticFormula;
use axum::{routing::get, Router};

pub static PORT: &'static str = "3002";

#[tokio::main]
async fn main() {
    println!("Hello, world from backend.");
    println!("{}", add(2, 3));
    
    let qf = QuadraticFormula::new(1.0, -3.0, 2.0);
    let ans = qf.calculate();
    println!("{:?}", ans);
    
    println!("{}", multiply(2, 3));

    // let app = Router::new().route("/", get(|| async { "Hello, World! backend James is the coolest." }));

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // http://127.0.0.1:3002
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}",PORT)).await.unwrap();
    // axum::serve(listener, app).await.unwrap();
    axum::serve(listener, create_router()).await.unwrap();
}

async fn hello_world() -> String {
    "Hello world_000000111".to_owned()
}

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        // .route("createQuadraticFormula", get(create_quadratic_formula))
        // .route("getOneQuadraticFormula", get(get_one_quadratic_formula))
}


#[cfg(test)]
mod test;
