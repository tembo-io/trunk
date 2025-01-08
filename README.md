<p align="center">
  <img src="https://github.com/tembo-io/trunk/assets/8935584/905ef1f3-10ff-48b5-90af-74af74ebb1b1" width=25% height=25%>
</p>

# Trunk

[![version](https://img.shields.io/crates/v/pg-trunk?label=CLI&logo=rust)](https://crates.io/crates/pg-trunk)
![](https://img.shields.io/badge/Ubuntu_22.x-201%20extensions-orange)
[![OSSRank](https://shields.io/endpoint?url=https://ossrank.com/shield/2643)](https://ossrank.com/p/2643)
[![Slack](https://img.shields.io/badge/%40trunk-community?logo=slack&label=slack)](https://join.slack.com/t/trunk-community/shared_invite/zt-1yiafma92-hFHq2xAN0ukjg_2AsOVvfg)

Trunk is an open source package manager and registry for PostgreSQL (Postgres) extensions:

- Visit [pgt.dev](https://pgt.dev) to discover and learn more about the building blocks of this rich ecosystem 
- Use the Trunk CLI ([pg-trunk](https://crates.io/crates/pg-trunk)) to build, publish and install Postgres extensions _of all kinds_.

![trunk-install-larger-font](https://github.com/tembo-io/trunk/assets/8935584/1c09e899-c77a-48c1-a978-a46f03774f1a)

## Trunk CLI

### Installation
To get started, run the following commands to download and leverage cargo to install Trunk's CLI.
1. `curl https://sh.rustup.rs -sSf | sh`
2. `cargo install pg-trunk`

### Usage

The Trunk CLI allows for building, publishing and installing Postgres extensions of all kinds. It abstracts away
complexities of extension development and management with the following commands:

### `trunk build`
Compile extensions of all kinds.

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

### `trunk publish`
Publish extensions to the registry, making them available to the Postgres community for discovery and installation.

```shell
❯ trunk publish pg_cron \
--version 1.6.4 \
--description "Job scheduler for PostgreSQL" \
--documentation "https://github.com/citusdata/pg_cron" \
--repository "https://github.com/citusdata/pg_cron" \
--license "PostgreSQL" \
--homepage "https://www.citusdata.com/"
```

### `trunk install`
Downloads Postgres extensions from the Trunk registry and installs into your environment (only Ubuntu support at this time).

Supports nested dependencies, e.g. installing `extension_a` will automatically install `extension_b` if required.

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

## Trunk Registry - https://pgt.dev
The Trunk registry serves as a community hub for Postgres extensions of all kinds. The Trunk CLI installs extensions and
their dependencies as compiled artifacts from this registry.

- Extension discovery and search
- Publish extensions for community use
- Install extensions in Postgres

It's our goal to develop Trunk to allow for:

- Usage metrics to provide insight into popular and well-maintained extensions
- Version tracking and new release email notifications

## ✨ Contributing
Trunk is in active development, and we look forward to the contributions the Postgres community has to offer.
If you're interested in contributing, please open a pull request, issue, or reach out!

- [Slack](https://join.slack.com/t/trunk-crew/shared_invite/zt-1yiafma92-hFHq2xAN0ukjg_2AsOVvfg)
- [Twitter](https://twitter.com/tembo_io)
- Email us at [hello@tembo.io](mailto:hello@tembo.io)

Thanks goes to these incredible people:

<a href="https://github.com/tembo-io/trunk/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=tembo-io/trunk" />
</a>
