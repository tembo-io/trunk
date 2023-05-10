# Trunk Registry - https://pgtrunk.io
The Trunk registry serves as the community's hub for Postgres extensions of all kinds. It stores extension information
in its database and compiled extension artifacts in AWS S3. Features include:

- Extension discovery and search
- Publish extensions for community use
- Install extensions in Postgres
- Usage metrics to provide insight into popular and well-maintained extensions
- Version tracking and new release email notifications

## Development
### Getting Started

1. Start postgres database
    ```
    docker run -it --rm -p 5432:5432 -e POSTGRES_PASSWORD=postgres postgres:latest
    ```

2. Set connection string environment variable. This can also be configured in `.env`.
    ```
    export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/postgres"
    ```

3. Initialize database (must install [sqlx](https://crates.io/crates/sqlx-cli))
    ```
    cargo sqlx migrate run
    ```

4. Run, with automatic reloads (uses [cargo watch](https://crates.io/crates/cargo-watch))
    ```
    cargo watch -x run
    ```

### Usage
The registry will run at `http://localhost:8080` by default. The [Trunk CLI](../cli) can be configured to interact with
a local registry by using the `--registry` flag. Example:
```shell
trunk publish pgmq \
--version 0.5.0 \
--description "Message queue for Postgres" \
--documentation "https://github.com/CoreDB-io/coredb/tree/main/pgmq/extension" \
--repository "https://github.com/CoreDB-io/coredb/tree/main/pgmq/extension" \
--license "Apache-2.0" \
--homepage "https://github.com/CoreDB-io/coredb" \
--registry http://localhost:8080
```

Routes can also be called with tools like `curl`. Examples:
```shell
curl --request GET --url 'http://localhost:8080/extensions/all'
```

```shell
curl --request GET --url 'http://localhost:8080/extensions/pgmq/0.5.0/download'
```

```shell
curl --request POST -H "Authorization: Bearer ${TOKEN}" --url 'http://localhost:8080/token/new'
```

```shell
curl -F metadata='{\
"name": "pgmq", \
"vers": "0.5.0", \
"description": "Message queue for Postgres", \
"documentation": "https://github.com/CoreDB-io/coredb/tree/main/pgmq/extension", \
"repository": "https://github.com/CoreDB-io/coredb/tree/main/pgmq/extension", \
"license": "Apache-2.0", \
"homepage": "https://coredb.io"}' \
-F file=@./pgmq-0.5.0.tar.gz --url 'http://localhost:8080/extensions/new'
```

## Architecture
The Trunk registry is made up of the following components:
- API
- PostgreSQL database
- AWS S3 bucket

![architecture.svg](../assets/architecture.svg)
