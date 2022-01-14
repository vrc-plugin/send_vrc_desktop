mod error;
mod handle;
mod router;
mod win32api;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = router::new();
    let addr = SocketAddr::from(([127, 0, 0, 1], 11400));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
