// axum imports
use axum::Router;

// sqlx imports
use sqlx::postgres::PgPool;

// std imports
use std::env;
use std::net::SocketAddr;

// misc imports
use dotenv::dotenv;

// mods
pub mod api {
    pub mod v1 {
        pub mod todos;
    }
}
mod routes;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db_pool: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let url = env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&url).await.unwrap();
    let state = AppState { db_pool: pool };

    let app = Router::new().with_state(state.clone()).merge(routes::configure(state));

    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening on {}", address);
    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
