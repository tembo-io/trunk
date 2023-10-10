use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use clerk_rs::{validators::actix::ClerkMiddleware, ClerkConfiguration};
use trunk_registry::repository::Registry;
use trunk_registry::routes::token::new_token;
use trunk_registry::{config, connect, routes};

pub fn routes_config(configuration: &mut web::ServiceConfig) {
    let cfg = config::Config::default();
    let clerk_cfg = ClerkConfiguration::new(None, None, Some(cfg.clerk_secret_key), None);
    configuration
        .service(routes::root::ok)
        .service(routes::extensions::get_all_extensions)
        .service(routes::extensions::get_version)
        .service(routes::extensions::get_version_history)
        .service(routes::extensions::get_shared_preload_libraries)
        .service(routes::categories::get_all_categories)
        .service(routes::extensions::publish)
        .service(routes::download::download)
        .service(
            web::scope("/token")
                .wrap(ClerkMiddleware::new(clerk_cfg.clone(), None, false))
                .service(new_token),
        )
        .service(
            web::scope("/admin")
                .wrap(ClerkMiddleware::new(clerk_cfg, None, false))
                .service(routes::root::auth_ok)
                .service(routes::extensions::delete_extension)
                .service(routes::extensions::put_shared_preload_libraries),
        );
}

pub async fn server() -> std::io::Result<()> {
    // load configurations from environment
    let cfg = config::Config::default();
    let aws_config = aws_config::load_from_env().await;

    let registry = Registry::connect(&cfg.database_url)
        .await
        .expect("failed to connect to database");

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
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(conn.clone()))
            .app_data(web::Data::new(registry.clone()))
            .app_data(web::Data::new(cfg.clone()))
            .app_data(web::Data::new(aws_config.clone()))
            .configure(routes_config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
