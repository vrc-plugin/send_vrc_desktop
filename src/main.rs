mod handle;
mod win32api;

use actix_web::{App, HttpServer};

use handle::url;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(url))
        .bind("127.0.0.1:11400")?
        .run()
        .await
}
