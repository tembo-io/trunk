Trunk CLI Development
=====================

The Trunk CLI test suite requires:

*   A local [Postgres] cluster, including development tools (specifically
    [`pg_config`]) in the path.
*   [Docker] to run containers that build extensions
*   The `tar` utility (for now)
*   Linux on `x86_64`; tests will pass on ARM platforms like macOS, but many
    will be skipped

To run the tests:

``` sh
cargo test -- --nocapture --test-threads=4
```

The `--nocapture` option ensures all output prints to the terminal. The
`--test-threads=4` limits the number of threads so as not to overwhelm Docker
(seems especially prone with macOS Docker's VM).

## Docker Desktop

Start a pgxn-tools privileged container with the repository directory mounted:

```sh
cd trunk
docker run -it --rm -v "$PWD:/repo" --privileged --platform linux/amd64 -w /repo pgxn/pgxn-tools bash
```

Inside the container, follow the [installation instructions] to set up the Apt repo:

``` sh
# Add Docker's official GPG key:
sudo apt-get update
sudo apt-get install ca-certificates curl
sudo install -m 0755 -d /etc/apt/keyrings
sudo curl -fsSL https://download.docker.com/linux/debian/gpg -o /etc/apt/keyrings/docker.asc
sudo chmod a+r /etc/apt/keyrings/docker.asc

# Add the repository to Apt sources:
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/debian \
  $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt-get update
```

Install Docker:

``` sh
sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
```

Fix [bug] and start Docker:

``` sh
perl -i -pe 's/ulimit -Hn/ulimit -n/' /etc/init.d/docker
systemctl start docker
```

Install and start Postgres and upgrade Rust:

``` sh
pg-start 15
rustup update
```

Run the tests:

``` sh
cd cli
cargo test -- --nocapture --test-threads=4
```

  [Postgres]: https://www.postgresql.org
    "PostgreSQL: The World's Most Advanced Open Source Relational Database"
  [`pg_config`]: https://www.postgresql.org/docs/current/app-pgconfig.html
    "PostgreSQL Docs: pg_config"
  [Docker]: https://www.docker.com
    "Docker: Accelerated Container Application Development"
  [installation instructions]: https://docs.docker.com/engine/install/debian/#install-using-the-repository
  [bug]: https://forums.docker.com/t/etc-init-d-docker-62-ulimit-error-setting-limit-invalid-argument-problem/139424/2
