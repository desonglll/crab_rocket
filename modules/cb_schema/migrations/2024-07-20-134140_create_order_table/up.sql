-- Your SQL goes here
CREATE TABLE IF NOT EXISTS order_table (
    order_id SERIAL PRIMARY KEY,
    customer_id INTEGER REFERENCES customer_table(customer_id),
    order_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    total_amount DECIMAL(10, 2),
    status VARCHAR(50) DEFAULT 'PENDING'
);
INSERT INTO order_table (customer_id, order_date, total_amount, status) VALUES
(1, CURRENT_TIMESTAMP, 1499.98, 'PENDING'),
(2, CURRENT_TIMESTAMP, 499.99, 'COMPLETED');
