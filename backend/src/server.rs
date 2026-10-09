use std::sync::Arc;

use axum::{extract::State, http::{HeaderMap, StatusCode, header::AUTHORIZATION}};

pub struct Config {
    pub address: String
}

pub struct AppState{
    config : Config
}

async fn handle_auth_middleware(
    State(_state): State<Arc<AppState>>,
    headers: HeaderMap,
    request: axum::http::Request<axum::body::Body>,    
    next: axum::middleware::Next,
) -> Result<axum::response::Response, axum::http::StatusCode> 
{
    println!("handle_auth_middleware");

    let auth_header = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    println!("{}", auth_header);

    Ok(next.run(request).await)
}

pub async fn run(config: Config) -> Result<(), Box<dyn std::error::Error>>
{
    println!("server init...");
    println!("address: {}", config.address);

    let address = config.address.clone();

    let shared_state = std::sync::Arc::new(AppState{config});

    let protected_routes = axum::Router::new()
        .route("/protected_backend_health", axum::routing::get(protected_backend_health_handler))
        .route_layer(axum::middleware::from_fn_with_state(shared_state.clone(), handle_auth_middleware));

    let router = axum::Router::new()
        .route("/backend_health", axum::routing::get(backend_health_handler))
        .merge(protected_routes)
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(address).await?;

    println!("server starting...");

    axum::serve(listener, router).await?;

    Ok(())
}

async fn protected_backend_health_handler() -> String {
    println!("protected_backend_health_handler");

    "protected_backend_health OK".to_string()
}

async fn backend_health_handler() -> String {
    println!("backend_health_handler");
    
    "backend_health OK".to_string()
}