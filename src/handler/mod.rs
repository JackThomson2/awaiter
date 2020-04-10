use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

use crate::file_mapper::MappedFiles;
use crate::response::text::*;

#[inline]
pub async fn handle_request(stream: &mut TcpStream, cache: &MappedFiles) -> io::Result<()> {
    let mut buf = [0u8; 1024];

    if let Ok(read) = stream.read(&mut buf).await {
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);

        let res = req.parse(&buf[..read]).unwrap();

        if res.is_complete() {
            match req.path {
                Some(ref path) => {
                    return send_file_response(stream, path, cache).await;
                }
                None => println!("No path provided"),
            }
        }

        //let res = try_parse_headers(read);
        return send_file_response(stream, "NONE", cache).await;
    }
    send_text_response(stream, "<html><body>Hello World</body></html>").await?;
    Ok(())
}
