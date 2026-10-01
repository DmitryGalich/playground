pub struct Config {
    pub address: String
}

pub struct AppState {
    config: Config
}

pub async fn run(config: Config) {
    println!("server init");
    println!("{}", config.address);
    let address = config.address.clone();
    let shared_state = std::sync::Arc::new(AppState{config});

    let app = axum::Router::new()
        .route("/backend_health",axum::routing::get(backend_health_handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    
    println!("server start");
    axum::serve(listener, app).await.unwrap();
}

async fn backend_health_handler() -> String{
    "backend_health".to_string()
}