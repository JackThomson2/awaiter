use crate::file_mapper::MappedFiles;
use crate::statics::mime::TEXT;
use crate::statics::version::*;

use tokio::net::TcpStream;
use tokio::prelude::*;

use colored::Colorize;

use mime_guess::from_path;

use std::io;

pub async fn send_file_response(
    stream: &mut TcpStream,
    sending: &str,
    map: &MappedFiles,
) -> io::Result<()> {
    let mut sending = sending;
    if !map.contains_key(sending) {
        if sending != "/" && !map.contains_key("/index.html") {
            sending = "/404.html";
        } else if sending == "" {
            sending = "/index.html";
        }
        if !map.contains_key("/404.html") {
            return send_404_response(stream, map).await;
        }
    }

    println!(
        "[{}] {:8} Serving file '{}'",
        "*".purple(),
        "Server".purple().bold(),
        sending.green().underline()
    );

    let data = map.get(sending).unwrap();
    let data = data.value();

    let message  = format!("{} 200 \r\nServer: {}\r\nContent-Type: {}\r\nAccept-Ranges: bytes\n\rContent-Length: {} \r\n\r\n"
                           ,VERSION, NAME, data.mime, data.data.len());

    stream.write_all(message.as_bytes()).await?;
    stream.write_all(&data.data).await
}

pub async fn send_404_response(stream: &mut TcpStream, map: &MappedFiles) -> io::Result<()> {
    let message  = format!("{} 404 \r\nServer: {}\r\nContent-Type: {}\r\nAccept-Ranges: bytes\n\rContent-Length: 0 \r\n\r\n"
                           ,VERSION, NAME, TEXT);

    stream.write_all(message.as_bytes()).await
}

pub async fn send_text_response(stream: &mut TcpStream, sending: &str) -> io::Result<()> {
    let data = sending.as_bytes();

    let message  = format!("{} 200 \r\nServer: {}\r\nContent-Type: {}\r\nAccept-Ranges: bytes\n\rContent-Length: {} \r\n\r\n"
            ,VERSION, NAME, TEXT, data.len());

    stream.write_all(message.as_bytes()).await?;
    stream.write_all(data).await
}
