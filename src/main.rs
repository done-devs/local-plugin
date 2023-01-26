extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use proto_rust::provider::provider_server::ProviderServer;
use tonic::transport::Server;

mod database;
mod models;
mod schema;
mod service;
mod setup;

use service::LocalService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:7007".parse()?;

    let local_service = LocalService {
        id: "Local".to_string(),
        name: "Local".to_string(),
        description: "Stores tasks on your computer.".to_string(),
        icon: "user-home-symbolic".to_string(),
    };

    setup::init();

    Server::builder()
        .add_service(ProviderServer::new(local_service))
        .serve(addr)
        .await?;

    Ok(())
}
