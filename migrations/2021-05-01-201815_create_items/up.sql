CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    body VARCHAR NOT NULL,
    description VARCHAR,
    quantity INTEGER NOT NULL DEFAULT 1,
    complete BOOLEAN NOT NULL DEFAULT False
)
