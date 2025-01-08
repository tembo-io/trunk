# Trunk CLI

The Trunk CLI allows for building, publishing and installing Postgres extensions of all kinds. It abstracts away
complexities of extension development and management with the following commands:

- `trunk build` - Compiles extensions into publishable and installable artifacts.
- `trunk publish` - Publishes extensions to the Trunk registry, making them available to the Postgres community for discovery and
installation.
- `trunk install` - Downloads Postgres extensions from the [Trunk registry](https://pgtrunk.io/) and installs in whichever environment trunk is
run.

## Installation
The Trunk CLI is hosted at [crates.io](https://crates.io/crates/pg-trunk) and can be installed with `cargo`.

1. `curl https://sh.rustup.rs -sSf | sh`
2. `cargo install pg-trunk`

- To check you're version of Trunk, invoke `trunk --version`.
- To upgrade to the latest version of Trunk, run `cargo install pg-trunk`.

## `trunk build`
The `build` command allows for compiling and packaging Postgres extensions from source. Packaged extensions are written to
`.trunk/<extension-name>-<extension-version>.tar.gz`.

```shell
❯ trunk build --help
Build a PGRX or C based Postgres extension

Usage: trunk build [OPTIONS]

Options:
  -p, --path <PATH>
          The file path of the extension to build [default: .]
  -o, --output-path <OUTPUT_PATH>

  -v, --version <VERSION>

  -n, --name <NAME>

  -e, --extension_name <EXTENSION_NAME>

  -x, --extension_dependencies <EXTENSION_DEPENDENCIES>

  -s, --preload-libraries <PRELOAD_LIBRARIES>

  -P, --platform <PLATFORM>

  -d, --dockerfile <DOCKERFILE_PATH>

  -i, --install-command <INSTALL_COMMAND>

  -t, --test
          Run this extension's integration tests after building, if any are found
      --pg-version <PG_VERSION>
          The PostgreSQL version to build this extension against. Experimental for versions other than Postgres 15 [default: 15]
  -h, --help
          Print help
```

### PGRX Based Extensions

Extensions can be built in many ways, and [PGRX](https://github.com/tcdi/pgrx) allows for us to do so with Rust.
Trunk makes building and packaging PGRX based extensions easier than ever.

Example `trunk build` with PGRX based extension
[jsonschema](https://pgxn.org/dist/jsonschema/):

```shell
❯ trunk build --pg-version 17
info: Building from path .
info: Using Dockerfile at ./Dockerfile
info: Using install command /usr/bin/bash -c make install
Building with name jsonschema
Building with version 0.1.4
Building for PostgreSQL 17
.
.
.
Creating package at: ./.trunk/jsonschema-0.1.4-pg17.tar.gz
Fetching extension_name from control file: extension/jsonschema.control
Using extension_name: jsonschema
Create Trunk bundle:
	jsonschema.so
	licenses/LICENSE.md
	extension/jsonschema--0.1.0--0.1.1.sql
	extension/jsonschema--0.1.1--0.1.2.sql
	extension/jsonschema--0.1.2--0.1.3.sql
	extension/jsonschema--0.1.3--0.1.4.sql
	extension/jsonschema--0.1.4.sql
	extension/jsonschema.control
	manifest.json
Packaged to ./.trunk/jsonschema-0.1.4-pg17.tar.gz
```

### C & SQL Based Extensions

Extensions can also be written in C & SQL. Let's take a look at how we can build C & SQL based extensions with Trunk.

#### Example `trunk build` with C & SQL based extension [pg_cron](https://github.com/citusdata/pg_cron)
```shell
❯ trunk build --pg-version 17
info: Building from path .
info: Using Dockerfile at ./Dockerfile
info: Using install command /usr/bin/bash -c make -C pg_cron install
Building with name pg_cron
Building with version 1.6.4
Building for PostgreSQL 17
.
.
.
Creating package at: ./.trunk/pg_cron-1.6.4-pg17.tar.gz
Fetching extension_name from control file: extension/pg_cron.control
Using extension_name: pg_cron
Create Trunk bundle:
	pg_cron.so
	licenses/LICENSE
	extension/pg_cron--1.0--1.1.sql
	extension/pg_cron--1.0.sql
	extension/pg_cron--1.1--1.2.sql
	extension/pg_cron--1.2--1.3.sql
	extension/pg_cron--1.3--1.4.sql
	extension/pg_cron--1.4--1.4-1.sql
	extension/pg_cron--1.4-1--1.5.sql
	extension/pg_cron--1.5--1.6.sql
	extension/pg_cron.control
	manifest.json
Packaged to ./.trunk/pg_cron-1.6.4-pg17.tar.gz
```

Some extensions are part of larger projects and include Makefiles with references to parent directories.
Examples of such extensions include those found in [postgres/contrib](https://github.com/postgres/postgres/tree/master/contrib).
Trunk can help us build and package these types of extensions as well.

#### Example `trunk build` with C & SQL based extension [pg_stat_statements](https://github.com/postgres/postgres/tree/master/contrib/pg_stat_statements)

Create a custom Dockerfile named `Dockerfile.pg_stat_statements` at the root of the [postgres/contrib](https://github.com/postgres/postgres/tree/master/contrib)
repository:

```dockerfile
# Set up variables for build.
ARG PG_VERSION
FROM quay.io/coredb/c-builder:pg${PG_VERSION}

# Extension build dependencies
USER root
RUN apt-get update && apt-get install -y \
	build-essential \
	libreadline-dev \
	zlib1g-dev \
	flex bison \
	libxml2-dev \
	libxslt-dev \
	libssl-dev \
	libxml2-utils \
	xsltproc \
	ccache

# Clone repository and build extension.
ARG EXTENSION_NAME
ARG PG_RELEASE
RUN git clone --depth 1 --branch "${PG_RELEASE}" https://github.com/postgres/postgres.git \
	&& make -C postgres/contrib/${EXTENSION_NAME} USE_PGXS=1
```

Run `trunk build` with `--dockerfile` and `--install-command` flags:

```shell
❯ trunk build \
--name pg_stat_statements \
--version 1.11.0 \
--dockerfile Dockerfile.pg_stat_statements \
--install-command make -C contrib/pg_stat_statements USE_PGXS=1 install
Building with name pg_stat_statements
Building with version 1.11.0
info: Building from path .
info: Using Dockerfile.pg_stat_statements
info: Using install command /bin/sh -c make -C postgres/contrib/pg_stat_statements install USE_PGXS=1
Building for PostgreSQL 17
.
.
.
Creating package at: ./.trunk/pg_stat_statements-1.11.0-pg17.tar.gz
Fetching extension_name from control file: extension/pg_stat_statements.control
Using extension_name: pg_stat_statements
Create Trunk bundle:
	pg_stat_statements.so
	licenses/COPYRIGHT
	licenses/COPYRIGHT.~1~
	extension/pg_stat_statements--1.0--1.1.sql
	extension/pg_stat_statements--1.1--1.2.sql
	extension/pg_stat_statements--1.10--1.11.sql
	extension/pg_stat_statements--1.2--1.3.sql
	extension/pg_stat_statements--1.3--1.4.sql
	extension/pg_stat_statements--1.4--1.5.sql
	extension/pg_stat_statements--1.4.sql
	extension/pg_stat_statements--1.5--1.6.sql
	extension/pg_stat_statements--1.6--1.7.sql
	extension/pg_stat_statements--1.7--1.8.sql
	extension/pg_stat_statements--1.8--1.9.sql
	extension/pg_stat_statements--1.9--1.10.sql
	extension/pg_stat_statements.control
	manifest.json
Packaged to ./.trunk/pg_stat_statements-1.11.0-pg17.tar.gz
```

## `trunk publish`

The `publish` command allows you to publish your newly-minted Postgres extension to the Trunk registry.
```shell
❯ trunk publish --help
Publish a Postgres extension to the Trunk registry

Usage: trunk publish [OPTIONS] [NAME]

Arguments:
  [NAME]

Options:
  -e, --extension_name <EXTENSION_NAME>
  -x, --extension_dependencies <EXTENSION_DEPENDENCIES>
  -s, --preload_libraries <PRELOAD_LIBRARIES>
  -v, --version <VERSION>
  -f, --file <FILE>
  -d, --description <DESCRIPTION>
  -D, --documentation <DOCUMENTATION>
  -H, --homepage <HOMEPAGE>
  -l, --license <LICENSE>
  -r, --registry <REGISTRY>
  -R, --repository <REPOSITORY>
  -c, --category <CATEGORY>
  -h, --help                                             Print help
```

1. Sign in at the [Trunk registry](https://pgtrunk.io) and click `Profile`.

2. Under `API Token`, click `Create New Token`.

3. `export TRUNK_API_TOKEN=<your-new-token>`

4. Run the following from the same directory your extension is in:
```shell
❯ trunk publish pg_cron \
--version 1.6.4 \
--description "Job scheduler for PostgreSQL" \
--documentation "https://github.com/citusdata/pg_cron" \
--repository "https://github.com/citusdata/pg_cron" \
--license "PostgreSQL" \
--homepage "https://www.citusdata.com/"
```

## `trunk install`

The `install` command allows you to install Postgres extensions from the Trunk registry. Trunk will automatically install any
additional extension dependencies that exist in the registry.

```shell
❯ trunk install --help
Install a Postgres extension from the Trunk registry

Usage: trunk install [OPTIONS] <NAME>

Arguments:
  <NAME>

Options:
  -p, --pg-config <PG_CONFIG>
  -f, --file <FILE>
  -v, --version <VERSION>        [default: latest]
  -r, --registry <REGISTRY>      [default: https://registry.pgtrunk.io]
      --pg-version <PG_VERSION>  The PostgreSQL version for which this extension should be installed. Experimental for versions other than Postgres 15
  -s, --skip-dependencies        Skip dependency resolution
  -h, --help                     Print help
```

```shell
❯ trunk install pg_cron
Using pkglibdir: "/var/lib/postgresql/data/tembo/17/lib"
Using sharedir: "/var/lib/postgresql/data/tembo"
Using Postgres version: 17
info: Downloading from: https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/pg_cron/pg_cron-pg17-1.6.4.tar.gz
info: Dependent extensions to be installed: []
info: Installing pg_cron 1.6.4
[+] pg_cron.so => /var/lib/postgresql/data/tembo/17/lib
info: Skipping license file licenses/LICENSE
[+] extension/pg_cron--1.0--1.1.sql => /var/lib/postgresql/data/tembo
[+] extension/pg_cron--1.0.sql => /var/lib/postgresql/data/tembo
[+] extension/pg_cron--1.1--1.2.sql => /var/lib/postgresql/data/tembo
[+] extension/pg_cron--1.2--1.3.sql => /var/lib/postgresql/data/tembo
[+] extension/pg_cron--1.3--1.4.sql => /var/lib/postgresql/data/tembo
[+] extension/pg_cron--1.4--1.4-1.sql => /var/lib/postgresql/data/tembo
[+] extension/pg_cron--1.4-1--1.5.sql => /var/lib/postgresql/data/tembo
[+] extension/pg_cron--1.5--1.6.sql => /var/lib/postgresql/data/tembo
[+] extension/pg_cron.control => /var/lib/postgresql/data/tembo

***************************
* POST INSTALLATION STEPS *
***************************

Install the following system-level dependencies:
	On systems using apt:
		libpq5
		libc6

Add the following to your postgresql.conf:
	shared_preload_libraries = 'pg_cron'

Enable the extension with:
	CREATE EXTENSION IF NOT EXISTS pg_cron CASCADE;
```
