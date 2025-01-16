CREATE TABLE users (
    uuid CHAR(36) PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    role INTEGER NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at BIGINT NOT NULL
);