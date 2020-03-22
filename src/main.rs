mod handler;
mod statics;
mod response;
mod file_mapper;
mod settings;

use async_std::io;
use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task;

use settings::load_settings;
use file_mapper::get_mapped_files;
use handler::handle_request;

use colored::Colorize;

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

fn main() -> io::Result<()> {
    let settings = load_settings();
    let file_cache = get_mapped_files(&settings.serve_root);

    let http_address = format!("127.0.0.1:{}", settings.http_port);

    task::block_on(async {
        let listener = TcpListener::bind(http_address).await?;

        println!(
            "[{}] {:8} Listening for HTTP traffic on '{}'",
            "*".purple(),
            "Server".purple().bold(),
            listener.local_addr()?.to_string().green().underline()
        );

        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream = stream?;
            let cache = file_cache.clone();
            task::spawn(async {
                let copy = cache;
                handle_request(stream, &copy).await.unwrap();
            });
        }
        Ok(())
    })
}