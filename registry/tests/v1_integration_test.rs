use actix_web::{web, App};

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use actix_web::web::Bytes;
    use serde_json::Value;
    use trunk_registry::connect;
    use trunk_registry::repository::Registry;
    use trunk_registry::routes::root::ok;
    use trunk_registry::routes::token::new_token;
    use trunk_registry::token::check_token_input;
    use trunk_registry::v1::routes::{
        all_trunk_projects, insert_trunk_project, trunk_project_by_name_and_version,
        trunk_projects_by_name,
    };

    trait BodyTest {
        fn as_str(&self) -> &str;
    }

    impl BodyTest for Bytes {
        fn as_str(&self) -> &str {
            std::str::from_utf8(self).unwrap()
        }
    }

    #[actix_web::test]
    async fn test_v1_end_to_end() {
        tracing_subscriber::fmt::init();

        let dummy_jwt = "Bearer eyJhbGciOiJIUzI1NiIsImtpZCI6Imluc18yTzgzQnVQM2ZvS3dHc1o3Tks5b1pVT0lrNkQiLCJ0eXAiOiJKV1QifQ.eyJhenAiOiJodHRwOi8vbG9jYWxob3N0OjMwMDAiLCJleHAiOjE2ODM1OTU0ODMsImlhdCI6MTY4MzU5NTQyMywiaXNzIjoiaHR0cHM6Ly9lbGVjdHJpYy1jcmFwcGllLTkyLmNsZXJrLmFjY291bnRzLmRldiIsImp0aSI6Ijg3ZTFjOTc5MTBmYzA5N2E1MDlkIiwibmJmIjoxNjgzNTk1NDEzLCJzaWQiOiJzZXNzXzJQWEZHRU9pSWJvM2U5cUpqYk01c3BkdW1teSIsInN1YiI6InVzZXJfMlBIbVgzWVBqbmpOV1VsMTZMR1FUbGR1bW15IiwidXNlck5hbWUiOiJkdW1teSJ9.a70cMX7g_asjO4O5oG3ym16KTyuGRsy21fHScriZms0";

        std::env::set_var("GITHUB_TOKEN", "dummy");
        std::env::set_var("CLERK_SECRET_KEY", "dummy");

        let cfg = trunk_registry::config::Config::default();
        let conn = connect(&cfg.database_url)
            .await
            .expect("error connecting to database");

        sqlx::migrate!()
            .run(&conn)
            .await
            .expect("error running migrations");

        let registry = Registry::connect(&cfg.database_url).await.unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(conn.clone()))
                .app_data(web::Data::new(cfg.clone()))
                .app_data(web::Data::new(registry))
                .service(ok)
                .service(insert_trunk_project)
                .service(all_trunk_projects)
                .service(trunk_projects_by_name)
                .service(trunk_project_by_name_and_version)
                .service(web::scope("/token").service(new_token)),
        )
        .await;

        // Generate API token
        let req = test::TestRequest::post()
            .uri("/token/new")
            .insert_header(("Authorization", dummy_jwt))
            .to_request();
        let resp = test::call_service(&app, req).await;
        let token = test::read_body(resp).await;

        // Assert token is valid
        let t = String::from_utf8(token.to_vec()).unwrap();
        let valid_result = check_token_input(&t);
        assert!(matches!(valid_result, Ok(())));

        // Insert a Trunk project
        let req = test::TestRequest::post()
        .insert_header(("Authorization", dummy_jwt))
        .set_json(
            serde_json::from_str::<Value>(r##"{
        "name":"adminpack",
        "description":"adminpack serves as an auxilliary support to administrative management tools, such as pgAdmin. This extension provides additional functionality including, but not limited to remote management of server log files.",
        "documentation_link":"https://www.postgresql.org/docs/current/adminpack.html",
        "repository_link":"https://github.com/postgres/postgres/tree/master/contrib/adminpack",
        "version":"2.1.0",
        "postgres_versions":[
           15,
           14,
           16
        ],
        "extensions":[
           {
              "extension_name":"adminpack",
              "version":"2.1",
              "trunk_project_name":"adminpack",
              "dependencies_extension_names":null,
              "loadable_libraries":null,
              "configurations":null,
              "control_file":{
                 "absent":false,
                 "content":"# adminpack extension\ncomment = 'administrative functions for PostgreSQL'\ndefault_version = '2.1'\nmodule_pathname = '$libdir/adminpack'\nrelocatable = false\nschema = pg_catalog\n"
              }
           }
        ],
        "downloads":[
           {
              "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg14-2.1.0.tar.gz",
              "pg_version":14,
              "platform":"linux/amd64",
              "sha256":"2888b839144ec0a4f56c3a98e828884e1856f938ebffc3e0f2cdba59fc1d2db0"
           },
           {
              "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg15-2.1.0.tar.gz",
              "pg_version":15,
              "platform":"linux/amd64",
              "sha256":"19eb5541dbe872aa21f0670f7649f9bdc80cc434e91c9ff17cca7e922edfdd4f"
           },
           {
              "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg16-2.1.0.tar.gz",
              "pg_version":16,
              "platform":"linux/amd64",
              "sha256":"59d554f4d25e8ca95c6de06d6199290a114c7977f89e9adade4717d4ce05f8c4"
           }
        ]
     }
    "##).unwrap()
        ).uri("/trunk-projects").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        // Insert a Trunk project
        let req = test::TestRequest::post()
                .insert_header(("Authorization", dummy_jwt))
                .set_json(
                    serde_json::from_str::<Value>(r##"{
                "name":"adminpack",
                "description":"adminpack serves as an auxilliary support to administrative management tools, such as pgAdmin. This extension provides additional functionality including, but not limited to remote management of server log files.",
                "documentation_link":"https://www.postgresql.org/docs/current/adminpack.html",
                "repository_link":"https://github.com/postgres/postgres/tree/master/contrib/adminpack",
                "version":"2.2.0",
                "postgres_versions":[
                   15,
                   14,
                   16
                ],
                "extensions":[
                   {
                      "extension_name":"otherpack",
                      "version":"2.2",
                      "trunk_project_name":"adminpack",
                      "dependencies_extension_names":null,
                      "loadable_libraries":null,
                      "configurations":null,
                      "control_file":{
                         "absent":false,
                         "content":"# adminpack extension\ncomment = 'administrative functions for PostgreSQL'\ndefault_version = '2.1'\nmodule_pathname = '$libdir/adminpack'\nrelocatable = false\nschema = pg_catalog\n"
                      }
                   }
                ],
                "downloads":[
                   {
                      "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg14-2.1.0.tar.gz",
                      "pg_version":14,
                      "platform":"linux/amd64",
                      "sha256":"2888b839144ec0a4f56c3a98e828884e1856f938ebffc3e0f2cdba59fc1d2db0"
                   },
                   {
                      "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg15-2.1.0.tar.gz",
                      "pg_version":15,
                      "platform":"linux/amd64",
                      "sha256":"19eb5541dbe872aa21f0670f7649f9bdc80cc434e91c9ff17cca7e922edfdd4f"
                   },
                   {
                      "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg16-2.1.0.tar.gz",
                      "pg_version":16,
                      "platform":"linux/amd64",
                      "sha256":"59d554f4d25e8ca95c6de06d6199290a114c7977f89e9adade4717d4ce05f8c4"
                   }
                ]
             }
            "##).unwrap()
                ).uri("/trunk-projects").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        // Get latest version of all Trunk projects
        let req = test::TestRequest::get().uri("/trunk-projects").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let resp_body = test::read_body(resp).await;
        let resp_body = std::str::from_utf8(&resp_body).unwrap();

        assert_eq!(serde_json::from_str::<Value>(resp_body).unwrap(), serde_json::from_str::<Value>(
            r##"
            [
                {
                    "name":"adminpack",
                    "description":"adminpack serves as an auxilliary support to administrative management tools, such as pgAdmin. This extension provides additional functionality including, but not limited to remote management of server log files.",
                    "documentation_link":"https://www.postgresql.org/docs/current/adminpack.html",
                    "repository_link":"https://github.com/postgres/postgres/tree/master/contrib/adminpack",
                    "version":"2.2.0",
                    "postgres_versions":[
                        15,
                        14,
                        16
                    ],
                    "extensions":[
                        {
                            "extension_name":"otherpack",
                            "version":"2.2",
                            "trunk_project_name":"adminpack",
                            "dependencies_extension_names":null,
                            "loadable_libraries":null,
                            "configurations":null,
                            "control_file":{
                            "absent":false,
                            "content":"# adminpack extension\ncomment = 'administrative functions for PostgreSQL'\ndefault_version = '2.1'\nmodule_pathname = '$libdir/adminpack'\nrelocatable = false\nschema = pg_catalog\n"
                            }
                        }
                    ],
                    "downloads":[
                        {
                            "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg16-2.1.0.tar.gz",
                            "pg_version":16,
                            "platform":"linux/amd64",
                            "sha256":"59d554f4d25e8ca95c6de06d6199290a114c7977f89e9adade4717d4ce05f8c4"
                        },
                        {
                            "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg14-2.1.0.tar.gz",
                            "pg_version":14,
                            "platform":"linux/amd64",
                            "sha256":"2888b839144ec0a4f56c3a98e828884e1856f938ebffc3e0f2cdba59fc1d2db0"
                        },
                        {
                            "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg15-2.1.0.tar.gz",
                            "pg_version":15,
                            "platform":"linux/amd64",
                            "sha256":"19eb5541dbe872aa21f0670f7649f9bdc80cc434e91c9ff17cca7e922edfdd4f"
                        }
                    ]
                }
            ]
            "##
        ).unwrap());

        // Get latest version of all Trunk projects
        let req = test::TestRequest::get()
            .uri("/trunk-projects/adminpack")
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
        let resp_body = test::read_body(resp).await;
        let resp_body = std::str::from_utf8(&resp_body).unwrap();

        assert_eq!(serde_json::from_str::<Value>(resp_body).unwrap(), serde_json::from_str::<Value>(
            r##"
            [
                {
                   "name":"adminpack",
                   "description":"adminpack serves as an auxilliary support to administrative management tools, such as pgAdmin. This extension provides additional functionality including, but not limited to remote management of server log files.",
                   "documentation_link":"https://www.postgresql.org/docs/current/adminpack.html",
                   "repository_link":"https://github.com/postgres/postgres/tree/master/contrib/adminpack",
                   "version":"2.1.0",
                   "postgres_versions":[
                      15,
                      14,
                      16
                   ],
                   "extensions":[
                      {
                         "extension_name":"adminpack",
                         "version":"2.1",
                         "trunk_project_name":"adminpack",
                         "dependencies_extension_names":null,
                         "loadable_libraries":null,
                         "configurations":null,
                         "control_file":{
                            "absent":false,
                            "content":"# adminpack extension\ncomment = 'administrative functions for PostgreSQL'\ndefault_version = '2.1'\nmodule_pathname = '$libdir/adminpack'\nrelocatable = false\nschema = pg_catalog\n"
                         }
                      }
                   ],
                   "downloads":[
                      {
                         "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg16-2.1.0.tar.gz",
                         "pg_version":16,
                         "platform":"linux/amd64",
                         "sha256":"59d554f4d25e8ca95c6de06d6199290a114c7977f89e9adade4717d4ce05f8c4"
                      },
                      {
                         "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg14-2.1.0.tar.gz",
                         "pg_version":14,
                         "platform":"linux/amd64",
                         "sha256":"2888b839144ec0a4f56c3a98e828884e1856f938ebffc3e0f2cdba59fc1d2db0"
                      },
                      {
                         "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg15-2.1.0.tar.gz",
                         "pg_version":15,
                         "platform":"linux/amd64",
                         "sha256":"19eb5541dbe872aa21f0670f7649f9bdc80cc434e91c9ff17cca7e922edfdd4f"
                      }
                   ]
                },
                {
                   "name":"adminpack",
                   "description":"adminpack serves as an auxilliary support to administrative management tools, such as pgAdmin. This extension provides additional functionality including, but not limited to remote management of server log files.",
                   "documentation_link":"https://www.postgresql.org/docs/current/adminpack.html",
                   "repository_link":"https://github.com/postgres/postgres/tree/master/contrib/adminpack",
                   "version":"2.2.0",
                   "postgres_versions":[
                      15,
                      14,
                      16
                   ],
                   "extensions":[
                      {
                         "extension_name":"otherpack",
                         "version":"2.2",
                         "trunk_project_name":"adminpack",
                         "dependencies_extension_names":null,
                         "loadable_libraries":null,
                         "configurations":null,
                         "control_file":{
                            "absent":false,
                            "content":"# adminpack extension\ncomment = 'administrative functions for PostgreSQL'\ndefault_version = '2.1'\nmodule_pathname = '$libdir/adminpack'\nrelocatable = false\nschema = pg_catalog\n"
                         }
                      }
                   ],
                   "downloads":[
                      {
                         "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg16-2.1.0.tar.gz",
                         "pg_version":16,
                         "platform":"linux/amd64",
                         "sha256":"59d554f4d25e8ca95c6de06d6199290a114c7977f89e9adade4717d4ce05f8c4"
                      },
                      {
                         "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg14-2.1.0.tar.gz",
                         "pg_version":14,
                         "platform":"linux/amd64",
                         "sha256":"2888b839144ec0a4f56c3a98e828884e1856f938ebffc3e0f2cdba59fc1d2db0"
                      },
                      {
                         "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg15-2.1.0.tar.gz",
                         "pg_version":15,
                         "platform":"linux/amd64",
                         "sha256":"19eb5541dbe872aa21f0670f7649f9bdc80cc434e91c9ff17cca7e922edfdd4f"
                      }
                   ]
                }
            ]
            "##
        ).unwrap());

        // Use the query parameter to get only the adminpack extension
        let req = test::TestRequest::get()
            .uri("/trunk-projects?extension_name=adminpack")
            .to_request();
        let resp = test::call_service(&app, req).await;
        let resp_body = test::read_body(resp).await;
        let resp_body = std::str::from_utf8(&resp_body).unwrap();

        assert_eq!(
            serde_json::from_str::<Value>(resp_body).unwrap(),
            serde_json::from_str::<Value>(
               r##"
               [
                  {
                     "name":"adminpack",
                     "description":"adminpack serves as an auxilliary support to administrative management tools, such as pgAdmin. This extension provides additional functionality including, but not limited to remote management of server log files.",
                     "documentation_link":"https://www.postgresql.org/docs/current/adminpack.html",
                     "repository_link":"https://github.com/postgres/postgres/tree/master/contrib/adminpack",
                     "version":"2.2.0",
                     "postgres_versions":[
                        15,
                        14,
                        16
                     ],
                     "extensions":[
                        {
                           "extension_name":"otherpack",
                           "version":"2.2",
                           "trunk_project_name":"adminpack",
                           "dependencies_extension_names":null,
                           "loadable_libraries":null,
                           "configurations":null,
                           "control_file":{
                              "absent":false,
                              "content":"# adminpack extension\ncomment = 'administrative functions for PostgreSQL'\ndefault_version = '2.1'\nmodule_pathname = '$libdir/adminpack'\nrelocatable = false\nschema = pg_catalog\n"
                           }
                        }
                     ],
                     "downloads":[
                        {
                           "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg16-2.1.0.tar.gz",
                           "pg_version":16,
                           "platform":"linux/amd64",
                           "sha256":"59d554f4d25e8ca95c6de06d6199290a114c7977f89e9adade4717d4ce05f8c4"
                        },
                        {
                           "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg14-2.1.0.tar.gz",
                           "pg_version":14,
                           "platform":"linux/amd64",
                           "sha256":"2888b839144ec0a4f56c3a98e828884e1856f938ebffc3e0f2cdba59fc1d2db0"
                        },
                        {
                           "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg15-2.1.0.tar.gz",
                           "pg_version":15,
                           "platform":"linux/amd64",
                           "sha256":"19eb5541dbe872aa21f0670f7649f9bdc80cc434e91c9ff17cca7e922edfdd4f"
                        }
                     ]
                  }
               ]
               "##
            )
            .unwrap()
        );

        // Use the query parameter to get only the otherpack extension
        let req = test::TestRequest::get()
            .uri("/trunk-projects?extension_name=otherpack")
            .to_request();
        let resp = test::call_service(&app, req).await;
        let resp_body = test::read_body(resp).await;
        let resp_body = std::str::from_utf8(&resp_body).unwrap();

        assert_eq!(
            serde_json::from_str::<Value>(resp_body).unwrap(),
            serde_json::from_str::<Value>(
               r##"
               [
                  {
                     "name":"adminpack",
                     "description":"adminpack serves as an auxilliary support to administrative management tools, such as pgAdmin. This extension provides additional functionality including, but not limited to remote management of server log files.",
                     "documentation_link":"https://www.postgresql.org/docs/current/adminpack.html",
                     "repository_link":"https://github.com/postgres/postgres/tree/master/contrib/adminpack",
                     "version":"2.2.0",
                     "postgres_versions":[
                        15,
                        14,
                        16
                     ],
                     "extensions":[
                        {
                           "extension_name":"otherpack",
                           "version":"2.2",
                           "trunk_project_name":"adminpack",
                           "dependencies_extension_names":null,
                           "loadable_libraries":null,
                           "configurations":null,
                           "control_file":{
                              "absent":false,
                              "content":"# adminpack extension\ncomment = 'administrative functions for PostgreSQL'\ndefault_version = '2.1'\nmodule_pathname = '$libdir/adminpack'\nrelocatable = false\nschema = pg_catalog\n"
                           }
                        }
                     ],
                     "downloads":[
                        {
                           "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg16-2.1.0.tar.gz",
                           "pg_version":16,
                           "platform":"linux/amd64",
                           "sha256":"59d554f4d25e8ca95c6de06d6199290a114c7977f89e9adade4717d4ce05f8c4"
                        },
                        {
                           "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg14-2.1.0.tar.gz",
                           "pg_version":14,
                           "platform":"linux/amd64",
                           "sha256":"2888b839144ec0a4f56c3a98e828884e1856f938ebffc3e0f2cdba59fc1d2db0"
                        },
                        {
                           "link":"https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/adminpack/adminpack-pg15-2.1.0.tar.gz",
                           "pg_version":15,
                           "platform":"linux/amd64",
                           "sha256":"19eb5541dbe872aa21f0670f7649f9bdc80cc434e91c9ff17cca7e922edfdd4f"
                        }
                     ]
                  }
               ]
               "##
            ).unwrap()
        );
    }
}
