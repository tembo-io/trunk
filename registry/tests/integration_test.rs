use actix_web::{web, App};

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::to_bytes;
    use actix_web::test;
    use actix_web::web::Bytes;
    use sqlx;
    use trunk_registry::connect;
    use trunk_registry::routes::categories::get_all_categories;
    use trunk_registry::routes::download::download;
    use trunk_registry::routes::extensions::get_all_extensions;
    use trunk_registry::routes::root::ok;
    use trunk_registry::routes::token::new_token;
    use trunk_registry::token::check_token_input;

    trait BodyTest {
        fn as_str(&self) -> &str;
    }

    impl BodyTest for Bytes {
        fn as_str(&self) -> &str {
            std::str::from_utf8(self).unwrap()
        }
    }

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
                .service(get_all_categories)
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

        // Test /categories/all
        let req = test::TestRequest::get().uri("/categories/all").to_request();
        let resp = test::call_service(&app, req).await;
        println!("status: {:?}", resp.response());
        let expected = "[\n  {\n    \"description\": \"Featured extension.\",\n    \"extension_count\": 0,\n    \"id\": 16,\n    \"name\": \"Featured\",\n    \"slug\": \"featured\"\n  },\n  {\n    \"description\": \"Monitoring and recording database activities.\",\n    \"extension_count\": 0,\n    \"id\": 2,\n    \"name\": \"Auditing / Logging\",\n    \"slug\": \"auditing_logging\"\n  },\n  {\n    \"description\": \"Tracking and application of database changes to targeted objects or processes.\",\n    \"extension_count\": 0,\n    \"id\": 3,\n    \"name\": \"Change Data Capture\",\n    \"slug\": \"change_data_capture\"\n  },\n  {\n    \"description\": \"Integration and interaction with external data sources, systems, and services.\",\n    \"extension_count\": 0,\n    \"id\": 4,\n    \"name\": \"Connectors\",\n    \"slug\": \"connectors\"\n  },\n  {\n    \"description\": \"Streamlining data loading, transformation processes, and basic data type management.\",\n    \"extension_count\": 0,\n    \"id\": 5,\n    \"name\": \"Data / Transformations\",\n    \"slug\": \"data_transformations\"\n  },\n  {\n    \"description\": \"Identifying and resolving issues.\",\n    \"extension_count\": 0,\n    \"id\": 6,\n    \"name\": \"Debugging\",\n    \"slug\": \"debugging\"\n  },\n  {\n    \"description\": \"Improving performance by targeting index use and creation, as well as database compaction and reorganization.\",\n    \"extension_count\": 0,\n    \"id\": 7,\n    \"name\": \"Index / Table Optimizations\",\n    \"slug\": \"index_table_optimizations\"\n  },\n  {\n    \"description\": \"Incorporating machine learning capabilities into a PostgreSQL database.\",\n    \"extension_count\": 0,\n    \"id\": 8,\n    \"name\": \"Machine Learning\",\n    \"slug\": \"machine_learning\"\n  },\n  {\n    \"description\": \"Metrics - Presenting performance indicators, such as cache and tuple-level statistics, process information, session-level activity, and more.\",\n    \"extension_count\": 0,\n    \"id\": 9,\n    \"name\": \"Metrics\",\n    \"slug\": \"metrics\"\n  },\n  {\n    \"description\": \"Ongoing database management related, but not limited to operations, deployment, or clusters.\",\n    \"extension_count\": 0,\n    \"id\": 10,\n    \"name\": \"Orchestration\",\n    \"slug\": \"orchestration\"\n  },\n  {\n    \"description\": \"Enabling efficient management, manipulation, and adaptation database logic.\",\n    \"extension_count\": 0,\n    \"id\": 11,\n    \"name\": \"Procedural Languages\",\n    \"slug\": \"procedural_languages\"\n  },\n  {\n    \"description\": \"Augmenting experience surrounding query metrics observability and usability.\",\n    \"extension_count\": 0,\n    \"id\": 12,\n    \"name\": \"Query Optimizations\",\n    \"slug\": \"query_optimizations\"\n  },\n  {\n    \"description\": \"Facilitating more efficient search operations within a database.\",\n    \"extension_count\": 0,\n    \"id\": 13,\n    \"name\": \"Search\",\n    \"slug\": \"search\"\n  },\n  {\n    \"description\": \"Employing defense strategies for data and databases.\",\n    \"extension_count\": 0,\n    \"id\": 14,\n    \"name\": \"Security\",\n    \"slug\": \"security\"\n  },\n  {\n    \"description\": \"Extending user management and database system oversight, as well as \\\"under-the-hood\\\" access to logic modification and external resources.\",\n    \"extension_count\": 0,\n    \"id\": 15,\n    \"name\": \"Tooling / Admin\",\n    \"slug\": \"tooling_admin\"\n  },\n  {\n    \"description\": \"Interrogating data to extract meaningful insights.\",\n    \"extension_count\": 0,\n    \"id\": 1,\n    \"name\": \"Analytics\",\n    \"slug\": \"analytics\"\n  }\n]";
        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body.as_str(), expected);
    }
}
