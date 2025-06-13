use super::router::Router;
use http::httprequest::HttpRequest;
use std::str;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }

    // 'a is already a part of &self here, which equals to &'a self
    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let connection_listener = TcpListener::bind(self.socket_addr).await?;

        println!("Running on {}", self.socket_addr);

        loop {
            let (stream, _) = connection_listener.accept().await?;

            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream).await {
                    eprintln!("Error handling connection: {}", e);
                }
            });
        }
    }
}

async fn handle_connection(
    mut stream: tokio::net::TcpStream,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Connection established");

    let mut read_buffer = [0; 4096]; // Increased buffer size
    let bytes_read = stream.read(&mut read_buffer).await?;

    let request_str = String::from_utf8_lossy(&read_buffer[..bytes_read]);
    let req: HttpRequest = request_str.to_string().into();

    Router::route(req, &mut stream).await?;

    Ok(())
}
