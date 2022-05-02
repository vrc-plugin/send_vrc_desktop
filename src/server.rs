mod error;
mod handler;
mod router;

use std::env;
use std::net::SocketAddr;

use anyhow::Result;
use time::OffsetDateTime;

const DEFAULT_PORT: u16 = 11400;

pub async fn start() -> Result<()> {
    let app = router::new();

    let port = env::var("PORT")
        .map(|p| p.parse().unwrap_or(DEFAULT_PORT))
        .unwrap_or(DEFAULT_PORT);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let local_datetime = OffsetDateTime::now_local().unwrap_or(OffsetDateTime::now_utc());

    println!("listening on {addr}");
    println!("START {local_datetime}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
