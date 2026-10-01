pub struct Config {
    pub address: String
}

pub struct AppState {
    config: Config
}

pub async fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("server init...");
    println!("{}", config.address);

    let address = config.address.clone();

    let shared_state = std::sync::Arc::new(AppState{config});

    let app = axum::Router::new()
        .route("/backend_health",axum::routing::get(backend_health_handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(address).await?;
    
    println!("server start");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn backend_health_handler(axum::extract::State(_state): axum::extract::State<std::sync::Arc<AppState>>,) -> String{
    "backend_health".to_string()
}