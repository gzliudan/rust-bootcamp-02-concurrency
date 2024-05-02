use anyhow::Result;
use std::{io, net::SocketAddr};
use tokio::{io::AsyncWriteExt, net::TcpStream};
use tracing::{info, warn};

const BUFFER_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:6379";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("redis listening on: {}", addr);

    loop {
        let (stream, remote_addr) = listener.accept().await?;
        info!("Accepted connection from: {}", remote_addr);

        tokio::spawn(async move {
            if let Err(e) = process_redis_connection(stream, remote_addr).await {
                warn!(
                    "Error processing connection {remote_addr}: {}",
                    e.to_string()
                );
            }
        });
    }
}

async fn process_redis_connection(mut stream: TcpStream, addr: SocketAddr) -> Result<()> {
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUFFER_SIZE);
        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {} bytes", n);
                let line = String::from_utf8_lossy(&buf);
                info!("line = {:?}", line);
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    info!("connection {addr} closed");
    Ok(())
}
