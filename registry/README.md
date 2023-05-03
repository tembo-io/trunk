# pgtrunk.io
The community's registry for Postgres extensions.

Features will include:

* Extension search and browsing
* Usage and release metrics, to provide insight into popular and well-maintained extensions
* User comments and social media streams
* Benchmarks and tests
* Version tracking and new release email notifications

## Development

Start postgres database
```
docker run -it --rm -p 5432:5432 -e POSTGRES_PASSWORD=postgres postgres:latest
```

Set connection string
```
export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/postgres"
```

Initialize database (must install [sqlx](https://crates.io/crates/sqlx-cli))
```
cargo sqlx migrate run
```

Run, with automatic reloads (uses [cargo watch](https://crates.io/crates/cargo-watch))
```
cargo watch -x run
```
