mod handler;
mod router;
mod server;
use server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new("127.0.0.1:8080");
    server.run().await?;
    Ok(())
}
