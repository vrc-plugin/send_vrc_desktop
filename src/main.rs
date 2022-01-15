mod server;
mod win32api;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    server::start().await
}
