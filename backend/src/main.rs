mod server;

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
    println!("backend init...");

    let config = server::Config {
        address: String::from("0.0.0.0:8080"),
    };
    
    server::run(config).await?;
    Ok(())
}
