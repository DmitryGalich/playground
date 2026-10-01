use std::{error, println};

pub struct Config {
    pub address: String
}

pub struct AppState{
    config : Config
}

pub async fn run(config: Config) -> Result<(), Box<dyn std::error::Error>>
{
    println!("server init...");
    println!("address: {}", config.address);

    let address = config.address.clone();

    let shared_state = std::sync::Arc::new(AppState{config});

    let router = axum::Router::new()
        .route("/backend_health", axum::routing::get(backend_health_handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(address).await?;

    println!("server starting...");

    axum::serve(listener, router).await?;

    Ok(())
}

async fn backend_health_handler() -> String {
    println!("backend_health_handler");
    
    "backend_health OK".to_string()
}