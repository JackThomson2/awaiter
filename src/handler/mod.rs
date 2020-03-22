mod header;

use async_std::io;
use async_std::net::TcpStream;
use async_std::prelude::*;

use header::try_parse_headers;

use crate::file_mapper::MappedFiles;
use crate::response::text::*;

#[inline]
pub async fn handle_request(stream: TcpStream, cache: &MappedFiles) -> io::Result<()> {
    let mut reader = stream.clone();
    let mut buf = vec![0u8; 1024];

    if let Ok(n) = reader.read(&mut buf).await {
        let read = std::str::from_utf8(&buf[..n]);
        if let Ok(read) = read {
            let res = try_parse_headers(read);
            return send_file_response(&mut reader, &res.location, cache).await
        }
    }
    send_text_response(&mut reader, "<html><body>Hello World</body></html>").await?;
    Ok(())
}