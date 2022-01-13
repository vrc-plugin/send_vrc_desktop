mod handle;
mod win32api;
use chrono::{DateTime, Local};

use actix_web::{App, HttpServer};

use handle::url;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let local_datetime: DateTime<Local> = Local::now();
    println!("START {}", local_datetime);

    HttpServer::new(|| App::new().service(url))
        .bind("127.0.0.1:11400")?
        .run()
        .await
}
