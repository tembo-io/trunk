<p align="center">
  <img src="https://github.com/tembo-io/trunk/assets/8935584/905ef1f3-10ff-48b5-90af-74af74ebb1b1" width=25% height=25%>
</p>

# Trunk

[![version](https://img.shields.io/crates/v/pg-trunk?label=CLI&logo=rust)](https://crates.io/crates/pg-trunk)
[![OSSRank](https://shields.io/endpoint?url=https://ossrank.com/shield/2643)](https://ossrank.com/p/2643)

Trunk is an open source package manager and registry for Postgres extensions. Use the Trunk CLI to build, publish
and install Postgres extensions _of all kinds_.

![trunk-install-larger-font](https://github.com/tembo-io/trunk/assets/8935584/1c09e899-c77a-48c1-a978-a46f03774f1a)

## Installation
The Trunk CLI is hosted at crates.io and can be installed with cargo.
1. `curl https://sh.rustup.rs -sSf | sh`
2. `cargo install pg-trunk`


## Trunk CLI
The Trunk CLI allows for building, publishing and installing Postgres extensions of all kinds. It abstracts away
complexities of extension development and management with the following commands:

### `trunk build`
- Compile extensions of all kinds.
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

### `trunk publish`
- Publish extensions to the registry, making them available to the Postgres community for discovery and installation.

```shell
❯ trunk publish pgmq \
--version 0.5.0 \
--description "Message Queue for postgres" \
--documentation "https://github.com/tembo-io/pgmq" \
--repository "https://github.com/tembo-io/pgmq" \
--license "Apache-2.0" \
--homepage "https://www.tembo.io"
```

### `trunk install`
- Downloads Postgres extensions from the Trunk registry and installs in whichever environment trunk is run.
- Supports nested dependencies, e.g. installing `extension_a` will automatically install `extension_b` if required.

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

## Trunk Registry - https://pgt.dev
The Trunk registry serves as the community's hub for Postgres extensions of all kinds. The Trunk CLI installs extensions and
their dependencies as compiled artifacts from this registry.

- Extension discovery and search
- Publish extensions for community use
- Install extensions in Postgres
- Usage metrics to provide insight into popular and well-maintained extensions
- Version tracking and new release email notifications

## Contributing
Trunk is in active development, and we look forward to the contributions the Postgres community has to offer.
If you're interested in contributing, please open a pull request, issue, or reach out:

- [Slack](https://join.slack.com/t/trunk-crew/shared_invite/zt-1yiafma92-hFHq2xAN0ukjg_2AsOVvfg)
- [Twitter](https://twitter.com/tembo_io)
- Email us at [hello@tembo.io](mailto:hello@tembo.io)
