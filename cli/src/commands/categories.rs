// This file is to be used as a local definition for valid category slugs. It is used in case we
// fail to fetch valid categories from the trunk registry (src/commands/publish.rs:61)

pub const VALID_CATEGORY_SLUGS: [&str; 15] = [
    "analytics",
    "auditing_logging",
    "change_data_capture",
    "connectors",
    "data_transformations",
    "debugging",
    "index_table_optimizations",
    "machine_learning",
    "metrics",
    "orchestration",
    "procedural_languages",
    "query_optimizations",
    "search",
    "security",
    "tooling_admin",
];
