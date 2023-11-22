use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use clerk_rs::{validators::actix::ClerkMiddleware, ClerkConfiguration};
use trunk_registry::openapi::build_docs;
use trunk_registry::readme::GithubApiClient;
use trunk_registry::repository::Registry;
use trunk_registry::routes::token::new_token;
use trunk_registry::{config, connect, routes, v1};
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

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
        .service(routes::readmes::fetch_and_save_readme)
        .service(routes::readmes::get_readme)
        .service(
            web::scope("/token")
                .wrap(ClerkMiddleware::new(clerk_cfg.clone(), None, false))
                .service(new_token),
        )
        .service(
            web::scope("/admin")
                .wrap(ClerkMiddleware::new(clerk_cfg.clone(), None, false))
                .service(routes::root::auth_ok)
                .service(routes::extensions::delete_extension)
                .service(routes::extensions::put_shared_preload_libraries),
        )
        .service(
            web::scope("/api/v1")
                .service(v1::routes::all_trunk_projects)
                .service(v1::routes::trunk_projects_by_name)
                .service(v1::routes::trunk_project_by_name_and_version),
        )
        .service(
            web::scope("/admin/api/v1")
                .wrap(ClerkMiddleware::new(clerk_cfg, None, false))
                .service(v1::routes::insert_trunk_project),
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
            .app_data(web::Data::new(GithubApiClient::new(
                cfg.github_token.clone(),
            )))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", build_docs()),
            )
            .service(Redoc::with_url("/redoc", build_docs()))
            .configure(routes_config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
