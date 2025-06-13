mod handler;
mod router;
mod server;
use server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new("localhost:3000");
    server.run().await?;
    Ok(())
}
