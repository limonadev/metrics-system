-- Your SQL goes here
CREATE TABLE ratings (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    book_id VARCHAR NOT NULL REFERENCES books(id),
    rating FLOAT NOT NULL
)