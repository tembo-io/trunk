# Trunk
[![milestone-alpha](https://img.shields.io/badge/milestone-alpha-orange)]()
[![OSSRank](https://shields.io/endpoint?url=https://ossrank.com/shield/2643)](https://ossrank.com/p/2643)
[![Discord Chat](https://img.shields.io/discord/1060568981725003789?label=Discord)][Discord]

Trunk is an open source package installer and registry for PostgreSQL extensions. Use the Trunk CLI to build, publish
and install PostgreSQL extensions _of all kinds_.

## Trunk CLI
The Trunk CLI allows for building, publishing and installing Postgres extensions of all kinds. It abstracts away
complexities of extension development and management with the following commands:

### `trunk build`
- Compile extensions of all kinds.
```bash
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
- Publish extensions to the registry, making it available to the Postgres community for discovery and installation.

```bash
❯ trunk publish pgmq \
--version 0.5.0 \
--description "Message Queue for postgres" \
--documentation "https://coredb-io.github.io/coredb/extensions/pgmq/" \
--repository "https://github.com/CoreDB-io/coredb" \
--license "Apache-2.0" \
--homepage "https://github.com/CoreDB-io/coredb"
```

### `trunk install`
- Download and install the extension distribution, in whichever environment trunk is run.
- Supports nested dependencies, e.g. installing `extension_a` will automatically install `extension_b` if required.

```bash
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

## Trunk Registry - https://pgtrunk.io
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
- [Discord](https://discord.com/channels/1060568981725003789/1089363774357647370)
- [Twitter](https://twitter.com/coredb_io)
- Email us at [hello@coredb.io](mailto:hello@coredb.io)

[Discord]: https://discord.com/channels/1060568981725003789/1089363774357647370
