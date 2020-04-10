mod file_mapper;
mod handler;
mod response;
mod settings;
mod statics;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::task;

use file_mapper::get_mapped_files;
use handler::handle_request;
use settings::load_settings;

use colored::Colorize;

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = load_settings();
    let file_cache = get_mapped_files(&settings.serve_root);

    let http_address = format!("0.0.0.0:{}", settings.http_port);
    let mut listener = TcpListener::bind(http_address).await?;

    println!(
        "[{}] {:8} Listening for HTTP traffic on '{}'",
        "*".purple(),
        "Server".purple().bold(),
        listener.local_addr()?.to_string().green().underline()
    );

    loop {
        let (mut socket, _) = listener.accept().await?;
        let cache = file_cache.clone();

        tokio::spawn(async move {
            let copy = cache;
            handle_request(&mut socket, &copy).await.unwrap();
        });
    }
}
