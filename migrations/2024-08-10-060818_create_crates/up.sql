-- Your SQL goes here
CREATE TABLE crates (
    id SERIAL PRIMARY KEY,
    rustaceans_id INTEGER NOT NULL REFERENCES rustaceans(id),
    name VARCHAR NOT NULL,
    code VARCHAR NOT NULL,
    version VARCHAR NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);