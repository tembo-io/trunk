# Introducing Trunk

Trunk serves as a registry where users can publish, search, and download community-made Postgres extensions. Inspired by popular developer hubs, such as [crates.io](http://crates.io) (Rust), [pypi.org](http://pypi.org) (Python), and [npmjs.com](http://npmjs.com) (JavaScript), Trunk aims to foster an information-rich environment. Here, developers can interact with the registry in a variety of ways and proudly showcase their contributions. Furthermore, users can gain insights into valuable metrics on extension downloads and trends.

At its core, the goal of trunk is to cultivate a thriving Postgres extension ecosystem by lowering the barriers to building, sharing, and using Postgres extensions.

# Roadmap

The Trunk infrastructure can be divided into the following: a command line interface (CLI), a registry, and a website.

### The CLI

The CLI toolkit will abstract away many complexities in extension development and installation by using the following commands:

`trunk init`
- setup your environment to build a new Postrgres extension.

`trunk test`
- facilitate the automated unit and integration testing Postgres extensions.

`trunk build`
- compiles extensions.
- supports nested dependencies, e.g. installing `extension_a` will automatically install `extension_b` if required.

`trunk publish`
- publishes an extension to the registry, making it available for discovery and installation.

`trunk install`
- download and install the extension distribution, in whichever environment trunk is run.

### The Registry

To complement the CLI, we are building a public registry for distributing extension source code and compiled binaries matched to operating system, architecture, Postgres version, and extension version.

This purpose-built registry would provide a centralized location for developers to share extensions and for users to discover them.

### The Website

We will launch a [website](https://pgtrunk.io) to help developers both discover and learn about extensions. This website is key to our success, as it will drive attention, traffic, etc, and lead users to the CLI tool.

Features will include:

- Extension search and browsing
- Usage and release metrics, to provide insight into popular and well-maintained extensions
- User comments and social media streams
- Benchmarks and tests
- Version tracking and new release email notifications
