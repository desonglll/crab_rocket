-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE user_permission (
    id SERIAL PRIMARY KEY,
    role_id INTEGER NOT NULL,
    permission_name VARCHAR(255) NOT NULL,
    permission_description TEXT,
    resource VARCHAR(255) NOT NULL,
    action VARCHAR(50) NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    created_by VARCHAR(255),
    updated_by VARCHAR(255),
    notes TEXT,
    FOREIGN KEY (role_id) REFERENCES role_table(role_id)
);

-- 插入示例数据
INSERT INTO user_permission (
    role_id, permission_name, permission_description, resource, action, is_active, created_by, updated_by, notes
) VALUES
    (1, 'View Users', 'Can view user details', 'User Management', 'View', TRUE, 'admin', 'admin', 'Initial setup'),
    (1, 'Edit Users', 'Can edit user details', 'User Management', 'Edit', TRUE, 'admin', 'admin', 'Initial setup'),
    (2, 'View Reports', 'Can view financial reports', 'Reports', 'View', TRUE, 'admin', 'admin', 'Initial setup');
