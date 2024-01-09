# Trunk Registry - https://pgtrunk.io
The Trunk registry serves as the community's hub for Postgres extensions of all kinds. It stores extension information
in its database and compiled extension artifacts in AWS S3. Features include:

- Extension discovery and search
- Publish extensions for community use
- Install extensions in Postgres
- Usage metrics to provide insight into popular and well-maintained extensions
- Version tracking and new release email notifications

## Usage
### Extension Categories
Extensions are grouped according to developer-centric use cases:

`Analytics` - Interrogate data to extract meaningful insights.

`Auditing / Logging` - Monitor and record database activities.

`Change Data Capture` - Track and apply database changes to targeted objects or processes.

`Connectors` - Integrate and interact with external data sources, systems, and services.

`Data / Transformations` - Streamline data loading, transformation processes, and basic data type management.

`Debugging` - Identify and resolve issues.

`Index / Table Optimizations` - Improve performance by targeting index use and creation, as well as database compaction and reorganization.

`Machine Learning` - Incorporate machine learning capabilities.

`Metrics` - Spotlight performance indicators, such as cache and tuple-level statistics, process information,  session-level activity, and more.

`Monitoring` - Offer real-time or near-real-time database activity and performance.

`Orchestration` - Establish ongoing database management related, but not limited to operations, deployment, or clusters.

`Procedural Languages` - Enable efficient management, manipulation, and adaptation of database logic.

`Query Optimizations` - Augment query experiences surrounding metrics observability and usability.

`Search` - Facilitate more efficient search operations within a database.

`Security` - Employ defense strategies for data and databases, including encryption, measures to prevent unauthorized access, and with other associated safeguarding tactics.

`Tooling / Admin` - Extend user management and database system oversight, as well as “under-the-hood” access to logic modification and external resources.

## Development
### Getting Started

1. Start postgres
    ```
    just start-postgres
    ```

2. Set AWS environment variables
    ```
    export AWS_ACCESS_KEY_ID=<my-id>
    export AWS_SECRET_ACCESS_KEY=<my-key>
    export AWS_REGION=<my-region>
    export AWS_SESSION_TOKEN=<my-token>
    export S3_BUCKET=<my-bucket>
    ```

3. Initialize database (must install [sqlx](https://crates.io/crates/sqlx-cli))
    ```
    just run-migrations
    ```

4. Run the registry code
    ```
    just run
    ```

### Usage
The registry will run at `http://localhost:8080` by default. The [Trunk CLI](../cli) can be configured to interact with
a local registry by using the `--registry` flag. Example:
```shell
TRUNK_API_TOKEN=<my-token> trunk publish --registry http://localhost:8080
```

Routes can also be called with tools like `curl`. Examples:
```shell
curl --request GET --url 'http://localhost:8080/extensions/all'
```

```shell
curl --request GET --url 'http://localhost:8080/extensions/pgmq/0.5.0/download'
```

```shell
curl --request POST -H "Authorization: Bearer ${TOKEN}" --url 'http://localhost:8080/token/new'
```

```shell
curl -F metadata='{\
"name": "pgmq", \
"vers": "0.5.0", \
"description": "Message queue for Postgres", \
"documentation": "https://github.com/tembo-io/coredb/tree/main/pgmq/extension", \
"repository": "https://github.com/tembo-io/coredb/tree/main/pgmq/extension", \
"license": "Apache-2.0", \
"homepage": "https://coredb.io"}' \
-F file=@./pgmq-0.5.0.tar.gz --url 'http://localhost:8080/extensions/new'
```

## Architecture
The Trunk registry is made up of the following components:
- API
- PostgreSQL database
- AWS S3 bucket

![architecture.svg](../assets/architecture.svg)
