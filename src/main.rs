use axum::Router;
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::net::SocketAddr;

pub mod api {
    pub mod todos;
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

    let app = Router::new()
        .with_state(state.clone())
        .merge(routes::configure(state));

    let address = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on {}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
