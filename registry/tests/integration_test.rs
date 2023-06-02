use actix_web::{web, App};

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use sqlx;
    use trunk_registry::connect;
    use trunk_registry::routes::download::download;
    use trunk_registry::routes::extensions::{get_all_extensions, get_version_history};
    use trunk_registry::routes::root::ok;
    use trunk_registry::routes::token::new_token;
    use trunk_registry::token::check_token_input;

    /// make sure the webserver boots up
    #[actix_web::test]
    async fn test_get_all_extensions() {
        env_logger::init();

        let dummy_jwt = "Bearer eyJhbGciOiJIUzI1NiIsImtpZCI6Imluc18yTzgzQnVQM2ZvS3dHc1o3Tks5b1pVT0lrNkQiLCJ0eXAiOiJKV1QifQ.eyJhenAiOiJodHRwOi8vbG9jYWxob3N0OjMwMDAiLCJleHAiOjE2ODM1OTU0ODMsImlhdCI6MTY4MzU5NTQyMywiaXNzIjoiaHR0cHM6Ly9lbGVjdHJpYy1jcmFwcGllLTkyLmNsZXJrLmFjY291bnRzLmRldiIsImp0aSI6Ijg3ZTFjOTc5MTBmYzA5N2E1MDlkIiwibmJmIjoxNjgzNTk1NDEzLCJzaWQiOiJzZXNzXzJQWEZHRU9pSWJvM2U5cUpqYk01c3BkdW1teSIsInN1YiI6InVzZXJfMlBIbVgzWVBqbmpOV1VsMTZMR1FUbGR1bW15IiwidXNlck5hbWUiOiJkdW1teSJ9.a70cMX7g_asjO4O5oG3ym16KTyuGRsy21fHScriZms0";

        let cfg = trunk_registry::config::Config::default();
        let conn = connect(&cfg.database_url)
            .await
            .expect("error connecting to database");

        sqlx::migrate!()
            .run(&conn)
            .await
            .expect("error running migrations");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(conn.clone()))
                .app_data(web::Data::new(cfg.clone()))
                .service(ok)
                .service(get_all_extensions)
                .service(download)
                .service(web::scope("/token").service(new_token)),
        )
        .await;

        // Generate API token
        let req = test::TestRequest::post()
            .uri("/token/new")
            .insert_header(("Authorization", dummy_jwt.clone()))
            .to_request();
        let resp = test::call_service(&app, req).await;
        let token = test::read_body(resp).await;

        // Assert token is valid
        let t = String::from_utf8(token.to_vec()).unwrap();
        let valid_result = check_token_input(&t);
        assert!(matches!(valid_result, Ok(())));

        // TODO(ianstanton) Publish dummy extension

        // Test /extensions/all
        let req = test::TestRequest::get().uri("/extensions/all").to_request();
        let resp = test::call_service(&app, req).await;
        println!("status: {:?}", resp.response());
        assert!(resp.status().is_success());

        let req = test::TestRequest::get()
            .uri("/extensions/get-all")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());

        // Test /
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        println!("status: {:?}", resp.response());
        assert!(resp.status().is_success());

        // Test /extensions/{extension_name}/{version}/download
        let req = test::TestRequest::get()
            .uri("/extensions/my_extension/0.0.1/download")
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("status: {:?}", resp.response());
        assert!(resp.status().is_success());

        // TODO(ianstanton) Test /extensions/detail/{extension_name}. Requires publishing dummy
        //  extension
    }
}
