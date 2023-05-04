use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use trunk_registry::{config, connect, download, publish, routes};

pub fn routes_config(cfg: &mut web::ServiceConfig) {
    cfg.service(routes::running)
        .service(routes::get_all_extensions)
        .service(publish::publish)
        .service(download::download);
}

pub async fn server() -> std::io::Result<()> {
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
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(conn.clone()))
            .app_data(web::Data::new(cfg.clone()))
            .app_data(web::Data::new(aws_config.clone()))
            .configure(routes_config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
