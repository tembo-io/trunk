CREATE TABLE v1.platform (
    id SERIAL PRIMARY KEY,
    platform_name VARCHAR(255) NOT NULL
);

INSERT INTO v1.platform (platform_name) VALUES ('linux/amd64');
INSERT INTO v1.platform (platform_name) VALUES ('linux/arm64');
