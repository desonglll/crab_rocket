-- Your SQL goes here
CREATE TABLE IF NOT EXISTS customer_table (
    customer_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    phone VARCHAR(20),
    address TEXT
);
INSERT INTO customer_table (name, email, phone, address) VALUES
('Alice Johnson', 'alice.johnson@example.com', '123-456-7890', '123 Maple St, Springfield'),
('Bob Smith', 'bob.smith@example.com', '987-654-3210', '456 Oak St, Springfield');
