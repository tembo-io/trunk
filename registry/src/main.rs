use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use trunk_registry::connect;
use trunk_registry::{config, download, publish, routes};
use trunk_registry::routes::new_token;
use clerk_rs::{ClerkConfiguration, validators::actix::ClerkMiddleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    // load configurations from environment
    let cfg = config::Config::default();
    let aws_config = aws_config::load_from_env().await;

    let conn = connect(&cfg.database_url)
        .await
        .expect("error connecting to database");

    // run database migrations
    sqlx::migrate!()
        .run(&conn)
        .await
        .expect("error running migrations");

    HttpServer::new(move || {
        let cors = Cors::permissive();
        let clerk_cfg = ClerkConfiguration::new(None, None, Some(cfg.clone().clerk_secret_key), None);
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(conn.clone()))
            .app_data(web::Data::new(cfg.clone()))
            .app_data(web::Data::new(aws_config.clone()))
            .service(routes::running)
            .service(routes::get_all_extensions)
            .service(publish::publish)
            .service(download::download)
            .service(
                web::scope("/token")
                    .wrap(ClerkMiddleware::new(clerk_cfg))
                    .service(new_token)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
