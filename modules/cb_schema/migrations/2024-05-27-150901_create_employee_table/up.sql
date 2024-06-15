-- Your SQL goes here
CREATE TABLE "employee_table" (
  "employee_id" serial,
  "first_name" varchar(255) DEFAULT '',
  "last_name" varchar(255) DEFAULT '',
  "employee_name" varchar(255) NOT NULL,
  "gender" varchar(255) DEFAULT '',
  "date_of_birth" timestamp DEFAULT CURRENT_TIMESTAMP,
  "hire_date" timestamp DEFAULT CURRENT_TIMESTAMP,
  "email" varchar(255) DEFAULT '',
  "phone_number" varchar(255) DEFAULT '',
  "department_id" int4 DEFAULT NULL,
  "job_title" varchar(255) DEFAULT '',
  "salary" int4 DEFAULT 0,
  "manager_id" int4 DEFAULT NULL,
  "address" varchar(255) DEFAULT '',
  "city" varchar(255) DEFAULT '',
  "state" varchar(255) DEFAULT '',
  "postal_code" varchar(255) DEFAULT '',
  "valid" bool DEFAULT true,
  "last_update" timestamp DEFAULT CURRENT_TIMESTAMP,
  "role_name" varchar(255) DEFAULT NULL,
  "role_id" int4 DEFAULT NULL,
  PRIMARY KEY ("employee_id"),
  CONSTRAINT "employee_name" UNIQUE ("employee_name")
);

-- 向 employee_table 表插入示例数据
INSERT INTO employee_table (first_name, last_name, employee_name, gender, date_of_birth, hire_date, email, phone_number, department_id, job_title, salary, manager_id, address, city, state, postal_code, role_name, role_id)
VALUES
('John', 'Doe', 'John Doe', 'Male', '1980-01-01', '2020-01-01', 'john.doe@example.com', '123-456-7890', 1, 'IT Specialist', 80000, NULL, '123 Main St', 'New York', 'NY', '10001', 'Admin', 1),
('Jane', 'Smith', 'Jane Smith', 'Female', '1985-05-15', '2019-02-01', 'jane.smith@example.com', '987-654-3210', 2, 'HR Manager', 90000, 1, '456 Elm St', 'Los Angeles', 'CA', '90001', 'User', 2),
('Mike', 'Johnson', 'Mike Johnson', 'Male', '1990-07-20', '2020-03-15', 'mike.johnson@example.com', '111-222-3333', 3, 'Financial Analyst', 85000, 2, '789 Oak St', 'Chicago', 'IL', '60601', 'Guest', 3);

ALTER TABLE "department_table" ADD CONSTRAINT "manager_id" FOREIGN KEY ("manager_id") REFERENCES "employee_table" ("employee_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "department_table" ADD CONSTRAINT "parent_department_id" FOREIGN KEY ("parent_department_id") REFERENCES "department_table" ("department_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "employee_table" ADD CONSTRAINT "department_id" FOREIGN KEY ("department_id") REFERENCES "department_table" ("department_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "employee_table" ADD CONSTRAINT "manager_id" FOREIGN KEY ("manager_id") REFERENCES "employee_table" ("employee_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "employee_table" ADD CONSTRAINT "role_name" FOREIGN KEY ("role_name") REFERENCES "role_table" ("role_name") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "employee_table" ADD CONSTRAINT "role_id" FOREIGN KEY ("role_id") REFERENCES "role_table" ("role_id") ON DELETE CASCADE ON UPDATE CASCADE;
