-- Add categories table
CREATE TABLE IF NOT EXISTS categories (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    slug VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    extension_count INT4 DEFAULT 0,
    created_at TIMESTAMP NOT NULL
);

-- Add category definitions
INSERT INTO categories(name, slug, description, created_at)
VALUES ('Analytics', 'analytics', 'Interrogating data to extract meaningful insights.', current_timestamp);
INSERT INTO categories(name, slug, description, created_at)
VALUES ('Auditing / Logging', 'auditing_logging', 'Monitoring and recording database activities.', current_timestamp);
INSERT INTO categories(name, slug, description, created_at)
VALUES ('Change Data Capture', 'change_data_capture', 'Tracking and application of database changes to targeted objects or processes.', current_timestamp);
INSERT INTO categories(name, slug, description, created_at)
VALUES ('Connectors', 'connectors', 'Integration and interaction with external data sources, systems, and services.', current_timestamp);
INSERT INTO categories(name, slug, description, created_at)
VALUES ('Data / Transformations', 'data_transformations', 'Streamlining data loading, transformation processes, and basic data type management.', current_timestamp);
INSERT INTO categories(name, slug, description, created_at)
VALUES ('Debugging', 'debugging', 'Identifying and resolving issues.', current_timestamp);
INSERT INTO categories(name, slug, description, created_at)
VALUES ('Index / Table Optimizations', 'index_table_optimizations', 'Improving performance by targeting index use and creation, as well as database compaction and reorganization.', current_timestamp);
INSERT INTO categories(name, slug, description, created_at)
VALUES ('Machine Learning', 'machine_learning', 'Incorporating machine learning capabilities into a PostgreSQL database.', current_timestamp);
INSERT INTO categories(name, slug, description, created_at)
VALUES ('Metrics', 'metrics', 'Metrics - Presenting performance indicators, such as cache and tuple-level statistics, process information, session-level activity, and more.', current_timestamp);
INSERT INTO categories(name, slug, description, created_at)
VALUES ('Orchestration', 'orchestration', 'Ongoing database management related, but not limited to operations, deployment, or clusters.', current_timestamp);
INSERT INTO categories(name, slug, description, created_at)
VALUES ('Procedural Languages', 'procedural_languages', 'Enabling efficient management, manipulation, and adaptation database logic.', current_timestamp);
INSERT INTO categories(name, slug, description, created_at)
VALUES ('Query Optimizations', 'query_optimizations', 'Augmenting experience surrounding query metrics observability and usability.', current_timestamp);
INSERT INTO categories(name, slug, description, created_at)
VALUES ('Search', 'search', 'Facilitating more efficient search operations within a database.', current_timestamp);
INSERT INTO categories(name, slug, description, created_at)
VALUES ('Security', 'security', 'Employing defense strategies for data and databases.', current_timestamp);
INSERT INTO categories(name, slug, description, created_at)
VALUES ('Tooling / Admin', 'tooling_admin', 'Extending user management and database system oversight, as well as "under-the-hood" access to logic modification and external resources.', current_timestamp);

-- Add extensions_categories table
CREATE TABLE IF NOT EXISTS extensions_categories (
    extension_id INT4 NOT NULL,
    category_id INT4 NOT NULL
);
