mod handler;
mod statics;
mod response;
mod file_mapper;

use async_std::io;
use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task;

use file_mapper::get_mapped_files;

use handler::handle_request;

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

fn main() -> io::Result<()> {
    let file_cache = get_mapped_files("./html");

    task::block_on(async {
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        println!("Listening on {}", listener.local_addr()?);

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