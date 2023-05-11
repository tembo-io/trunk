# Trunk CLI

The Trunk CLI allows for building, publishing and installing Postgres extensions of all kinds. It abstracts away
complexities of extension development and management with the following commands:
- `trunk build` - Compiles extensions of all kinds.
- `trunk publish` - Publish extensions to the registry, making it available to the Postgres community for discovery and
installation.
- `trunk install` - Downloads Postgres extensions from the Trunk registry and installs in whichever environment trunk is
`run.

## Installation
The Trunk CLI is hosted at [crates.io](https://crates.io/crates/pg-trunk) and can be installed with `cargo`.
1. `curl https://sh.rustup.rs -sSf | sh`
2. `cargo install pg-trunk`

To upgrade to the latest version of Trunk, run`cargo install pg-trunk`.


## `trunk build`

Usage depends on how the extension was written
pgrx

explain custom Dockerfile

Example `trunk build` with PGRX based extension:
```shell
❯ trunk build
Building from path .
Detected that we are building a pgrx extension
Detected pgrx version range 0.7.4
Using pgrx version 0.7.4
Building pgrx extension at path .
.
.
.
Creating package at: ./.trunk/pgmq-0.5.0.tar.gz
Create Trunk bundle:
	pgmq.so
	extension/pgmq--0.5.0.sql
	extension/pgmq.control
	manifest.json
Packaged to ./.trunk/pgmq-0.5.0.tar.gz
```
Packaged extensions are written to `.trunk/<extension-name>-<extension-version>.tar.gz`.

c + sql


## `trunk publish`

This command allows you to publish your newly-minted Postgres extension to the Trunk registry.

Trunk will look for your packaged extension file in your working directory with the naming format
`<extension-name>-<extension-version>.tar.gz`. Simply run the following from the same directory your extension is in:

```shell
❯ trunk publish pgmq \
--version 0.5.0 \
--description "Message Queue for postgres" \
--documentation "https://coredb-io.github.io/coredb/extensions/pgmq" \
--repository "https://github.com/CoreDB-io/coredb" \
--license "Apache-2.0" \
--homepage "https://www.coredb.io"
```

If the file is elsewhere, you can specify a filepath by using the `--file` flag.

## `trunk install`

This command allows you to install Postgres extensions from the Trunk registry. Trunk will automatically install any
additional extension dependencies, so long as they exist in the registry. Local files can be specified using the
`--file` flag.
```shell
❯ trunk install pgmq
Using pg_config: /usr/bin/pg_config
Using pkglibdir: "/usr/lib/postgresql/15/lib"
Using sharedir: "/usr/share/postgresql/15"
Downloading from: https://cdb-plat-use1-prod-pgtrunkio.s3.amazonaws.com/extensions/pgmq/pgmq-0.5.0.tar.gz
Dependencies: ["pg_partman"]
Installing pgmq 0.5.0
[+] pgmq.so => /usr/lib/postgresql/15/lib
[+] extension/pgmq--0.5.0.sql => /usr/share/postgresql/15
[+] extension/pgmq.control => /usr/share/postgresql/15
```
