use crate::server::server;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().compact().init();

    server().await
}
