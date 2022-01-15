mod error;
mod handler;
mod router;

use std::env;
use std::net::SocketAddr;

use anyhow::Result;

const DEFAULT_PORT: u16 = 11400;

pub async fn start() -> Result<()> {
    let app = router::new();

    let port = env::var("PORT")
        .map(|p| p.parse().unwrap_or(DEFAULT_PORT))
        .unwrap_or(DEFAULT_PORT);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
