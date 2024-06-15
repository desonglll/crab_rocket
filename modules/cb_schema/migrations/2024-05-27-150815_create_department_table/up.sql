-- Your SQL goes here
CREATE TABLE "department_table" (
  "department_id" serial,
  "department_name" varchar(255) NOT NULL,
  "manager_id" int4 DEFAULT NULL,
  "location" varchar(255) DEFAULT '',
  "creation_date" timestamp DEFAULT CURRENT_TIMESTAMP,
  "last_update" timestamp DEFAULT CURRENT_TIMESTAMP,
  "description" varchar(255) DEFAULT '',
  "budget" int4 DEFAULT 0,
  "number_of_employees" int4 DEFAULT 0,
  "parent_department_id" int4 DEFAULT NULL,
  "email" varchar(255) DEFAULT '',
  "phone_number" varchar(255) DEFAULT '',
  "address" varchar(255) DEFAULT '',
  "city" varchar(255) DEFAULT '',
  "state" varchar(255) DEFAULT '',
  "postal_code" varchar(255) DEFAULT '',
  PRIMARY KEY ("department_id"),
  CONSTRAINT "department_name" UNIQUE ("department_name")
);

-- 向 department_table 表插入示例数据
INSERT INTO department_table (department_name, manager_id, location, description, budget, number_of_employees, email, phone_number, address, city, state, postal_code)
VALUES
('IT Department', 1, 'New York', 'Information Technology Department', 100000, 10, 'it@example.com', '123-456-7890', '123 Main St', 'New York', 'NY', '10001'),
('HR Department', 2, 'Los Angeles', 'Human Resources Department', 80000, 8, 'hr@example.com', '987-654-3210', '456 Elm St', 'Los Angeles', 'CA', '90001'),
('Finance Department', 3, 'Chicago', 'Finance Department', 120000, 12, 'finance@example.com', '111-222-3333', '789 Oak St', 'Chicago', 'IL', '60601');

