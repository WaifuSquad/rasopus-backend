CREATE TABLE users (
    uuid UUID PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    role SMALLINT NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at BIGINT NOT NULL
);