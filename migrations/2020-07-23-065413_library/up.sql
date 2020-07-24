-- Your SQL goes here
CREATE TABLE "libraries" (
    id VARCHAR PRIMARY KEY,
    title VARCHAR NOT NULL,
    author VARCHAR NOT NULL,
    price NUMERIC CHECK (price >= 0) NOT NULL
)