-- Your SQL goes here
CREATE TABLE "lends" (
    username VARCHAR PRIMARY KEY REFERENCES users(username),
    books TEXT[] NOT NULL
)