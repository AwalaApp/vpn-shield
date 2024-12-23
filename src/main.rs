use anyhow::Result;
use awala_vpn_shield::create_router;

const NETLOC: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> Result<()> {
    let app = create_router();
    let listener = tokio::net::TcpListener::bind(NETLOC).await?;
    println!("Server running on http://{}", NETLOC);
    axum::serve(listener, app).await?;
    Ok(())
}
