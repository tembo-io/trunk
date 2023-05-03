use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use trunk_registry::connect;
use trunk_registry::{config, download, publish, routes};
use crate::server::server;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    return server().await;
}