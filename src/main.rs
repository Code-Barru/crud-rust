use std::sync::Arc;

use axum::Router;
use http::{header, Method};
use tokio::{net::TcpListener, sync::Mutex};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{self, TraceLayer},
};
use tracing::Level;
mod categories;
mod products;
mod users;

#[derive(Debug, Clone)]
pub struct AppState {
    pub users: Arc<Mutex<Vec<users::models::User>>>,
    pub products: Arc<Mutex<Vec<products::models::Product>>>,
    pub categories: Arc<Mutex<Vec<categories::models::Category>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        users: Arc::new(Mutex::new(Vec::new())),
        products: Arc::new(Mutex::new(Vec::new())),
        categories: Arc::new(Mutex::new(Vec::new())),
    };

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let cors: CorsLayer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
        ]);

    let app: Router = Router::new()
        .nest("/users", users::services::get_router(state.clone()))
        .nest("/products", products::services::get_router(state.clone()))
        .nest(
            "/categories",
            categories::services::get_router(state.clone()),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(cors);

    let listener: TcpListener = match TcpListener::bind("0.0.0.0:3000").await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind to port 3000: {}", e);
            return;
        }
    };

    match axum::serve(listener, app).await {
        Ok(_) => println!("Server exited successfully"),
        Err(e) => eprintln!("Server exited with an error: {}", e),
    };
}
