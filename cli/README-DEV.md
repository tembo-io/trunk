Trunk CLI Development
=====================

The Trunk CLI test suite requires:

*   A local [Postgres] cluster, including development tools (specifically
    [`pg_config`]) in the path.
*   [Docker] to run containers that build extensions
*   The `tar` utility (for now)
*   Linux on `x86_64`; test will pass on ARM platforms like macOS, but many will
    be skipped

To run the tests:

``` sh
cargo test -- --nocapture --test-threads=4
```

The `--nocapture` option ensures all output prints to the terminal. The
`--test-threads=4` limits the number of threads so as not to overwhelm Docker
(seems especially prone with macOS Docker's VM).

  [Postgres]: https://www.postgresql.org
    "PostgreSQL: The World's Most Advanced Open Source Relational Database"
  [`pg_config`]: https://www.postgresql.org/docs/current/app-pgconfig.html
    "PostgreSQL Docs: pg_config"
  [Docker]: https://www.docker.com
    "Docker: Accelerated Container Application Development"
