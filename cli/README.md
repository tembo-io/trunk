# Trunk CLI

The [Trunk Registry](https://pgtrunk.io) offers a companion CLI to facilitate a user-friendly programmatic interface. 
This toolset lowers the barriers to building, sharing, and using PostgreSQL extensions.

## Installation
The Trunk CLI is hosted at [crates.io](https://crates.io/crates/pg-trunk) and can be installed with `cargo`.
1. `curl https://sh.rustup.rs -sSf | sh`
2. `cargo install pg-trunk`

To upgrade to the latest version of Trunk, run`cargo install pg-trunk`.

## Commands

The CLI toolkit will abstract away many complexities in extension development and installation by using the following commands:
- `trunk build` - compiles extensions and supports nested dependencies.
- `trunk publish` - publishes an extension to the registry, making it available for discovery and installation.
- `trunk install` - download and install the extension distribution, in whichever environment trunk is run.

## `trunk build`

This command leverages [pgrx](https://github.com/tcdi/pgrx) to help you build compiled Postgres extensions. 

Usage: trunk build [OPTIONS]

Options:
- -p, --path <PATH>                [default: .]
- -o, --output-path <OUTPUT_PATH>  [default: ./.trunk]
- -h, --help                       Print help

## `trunk publish`

This command allows you to publish your newly-minted Postgres extension to the Trunk registry.

Usage: trunk publish [OPTIONS] --version <VERSION> <NAME>

Arguments:
  <NAME>

Options:
-  -v, --version <VERSION>
-  -f, --file <FILE>
-  -d, --description <DESCRIPTION>
-  -D, --documentation <DOCUMENTATION>
-  -H, --homepage <HOMEPAGE>
-  -l, --license <LICENSE>
-  -r, --registry <REGISTRY>            [default: https://registry.pgtrunk.io]
-  -R, --repository <REPOSITORY>
-  -h, --help                           Print help


## `trunk install`

This command allows you to install Postgres extensions from the Trunk registry.

Usage: trunk install [OPTIONS]< --version <VERSION> <NAME>

Arguments:
  <NAME>

Options:
-  -p, --pg-config <PG_CONFIG>
-  -f, --file <FILE>
-  -v, --version <VERSION>
-  -r, --registry <REGISTRY>    [default: https://registry.pgtrunk.io]
-  -h, --help                   Print help
