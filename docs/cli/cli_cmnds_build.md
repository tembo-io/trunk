# Build

The `build` command allows for compiling and packaging Postgres extensions from source. Packaged extensions are written to .trunk/<extension-name>-<extension-version>.tar.gz.

## Options
### -p, --path
Define a path that the Trunk CLI will use to search for relevant files.

- Default Behavior: If this option is not specified, the Trunk CLI operates in the directory where the build command is invoked.

### -o, --output-path
Sets the directory where the product is written to.

- Default Behavior: If this option is not specified, a new directory named .trunk is created in the current directory, resulting in ./.trunk.

### -v, --version
Use this option to specify the version of the extension. The version is usually associated with the highest value found in a SQL file. Trunk abides by semantic versioning standards. For more information, please refer to the [SOURCE LINK HERE].

- Default Behavior: It is required to include the extension version. Failing to do so will cause an error.

### -n, --name
This option allows you to define the name of your extension. While you have creative freedom, a good naming convention`pg_`.

- Default Behavior: Similar to --version, it is necessary to attribute a name to the extension you would like to build.

### -P, --platform
This option enables you to specify the target architecture that will be used when building the extension. This allows you to ensure compatibility of your extension with the desired systems. The current options are as follows:

- linux/amd64
- arm

### -d, --dockerfile

- Updates to come.
- Default Behavior: If this option is not specified and a Makefile is detected, the default Dockerfile at ./builders/Dockerfile.generic is used. If a Cargo.toml file is detected, the Dockerfile is not required.

### -i, --install-command
This option is used to specify the command that will be used to install the extension during the build process. In the context of this build script, if a Cargo.toml file is detected, the script assumes that it's building a pgrx extension and handles the build process internally. In other words, it does not require an install command. However, if a Makefile is detected, the script presumes that it is building an extension with make and make install. In this scenario, the --install-command becomes essential.

- Default Behavior: If this option is not specified, the default install command make install is used.
- Note: The --install-command is only used when building with a Makefile. The --version and --name options are mandatory in this case.

### --h, --help
This option displays a help message summarizing the usage of the command-line options.


## Example

### PGRX Based Extensions
Extensions can be built in many ways, and [PGRX](https://github.com/tcdi/pgrx) allows for us to do so with Rust.
Trunk makes building and packaging PGRX based extensions easier than ever.

Example `trunk build` with PGRX based extension
[pgmq](https://github.com/tembo-io/coredb/tree/main/pgmq/extension):
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
