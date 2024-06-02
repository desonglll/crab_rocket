-- Your SQL goes here
CREATE TABLE files (
    id UUID PRIMARY KEY,
    file_name VARCHAR NOT NULL,
    file_url VARCHAR NOT NULL,
    uploaded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
