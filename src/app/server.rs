use crate::app::routing::create_router;
use anyhow::Result;
use tokio::net::TcpListener;

const NETLOC: &str = "127.0.0.1:8080";

pub async fn run_server() -> Result<()> {
    let app = create_router();
    let listener = TcpListener::bind(NETLOC).await?;
    println!("Server running on http://{}", NETLOC);
    axum::serve(listener, app).await?;
    Ok(())
}
