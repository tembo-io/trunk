# Trunk (alpha)
Trunk serves as a registry where users can publish, search, and download community-made Postgres extensions. Inspired by
popular developer hubs, such as [crates.io](http://crates.io) (Rust), [pypi.org](http://pypi.org) (Python), and
[npmjs.com](http://npmjs.com) (JavaScript), Trunk aims to foster an information-rich environment. Here, developers can
interact with the registry in a variety of ways and proudly showcase their contributions. Furthermore, users can gain
insights into valuable metrics on extension downloads and trends.

At its core, the goal of trunk is to cultivate a thriving Postgres extension ecosystem by lowering the barriers to
building, sharing, and using Postgres extensions.

## Trunk CLI
The CLI toolkit will abstract away many complexities in extension development and installation by using the following
commands:

`trunk build`
- compiles extensions.

`trunk publish`
- publishes an extension to the registry, making it available for discovery and installation.

`trunk install`
- download and install the extension distribution, in whichever environment trunk is run.
- supports nested dependencies, e.g. installing `extension_a` will automatically install `extension_b` if required.

## Trunk Registry
To complement the CLI, we are building a public registry for distributing extension source code and compiled binaries
matched to operating system, architecture, Postgres version, and extension version.

This purpose-built registry would provide a centralized location for developers to share extensions and for users to
discover them.

### The Website
We will launch a [website](https://pgtrunk.io) to help developers both discover and learn about extensions. This website
is key to our success, as it will drive attention, traffic, etc, and lead users to the CLI tool.

Features will include:

- Extension search and browsing
- Usage and release metrics, to provide insight into popular and well-maintained extensions
- User comments and social media streams
- Benchmarks and tests
- Version tracking and new release email notifications

## Contributing
Trunk is in active development, and we look forward to the contributions the Postgres community has to offer.
If you're interested in contributing, please open a pull request, issue, or reach out:
- [Discord](https://discord.com/channels/1060568981725003789/1089363774357647370)
- [Twitter](https://twitter.com/coredb_io)
- Email us at [hello@coredb.io](mailto:hello@coredb.io)
