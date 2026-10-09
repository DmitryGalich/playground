mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("backend init...");

    let config = server::Config{
        address: "0.0.0.0:8000".to_string(),
    };

    server::run(config).await?;

    Ok(())
}
