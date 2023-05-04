use crate::server::server;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}
