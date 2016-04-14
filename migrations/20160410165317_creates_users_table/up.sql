CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    bio TEXT NOT NULL,
    password_digest BYTEA NOT NULL,
    salt BYTEA NOT NULL
)
