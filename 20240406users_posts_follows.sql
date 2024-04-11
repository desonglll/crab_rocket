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
  PRIMARY KEY ("employee_id"),
  CONSTRAINT "employee_name" UNIQUE ("employee_name")
);

CREATE TABLE "follows" (
  "following_user_id" int4 NOT NULL DEFAULT -1,
  "followed_user_id" int4 NOT NULL DEFAULT -1,
  "created_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "follow_id" serial,
  PRIMARY KEY ("follow_id")
);

CREATE TABLE "posts" (
  "post_id" serial,
  "title" varchar(255) DEFAULT 'Untitled',
  "body" text DEFAULT 'No Content',
  "user_id" int4 DEFAULT -1,
  "status" varchar(255) DEFAULT 'unknow status',
  "created_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "updated_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY ("post_id")
);

CREATE TABLE "tasks" (
  "id" serial,
  "title" text NOT NULL,
  "content" text DEFAULT '',
  "created_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "updated_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "user_id" int4 DEFAULT -1,
  PRIMARY KEY ("id")
);

CREATE TABLE "users" (
  "user_id" serial,
  "username" varchar(255) NOT NULL,
  "role" varchar(255) DEFAULT 'unknow role',
  "created_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "email" varchar(255) DEFAULT '',
  "password" varchar(255) NOT NULL,
  "fullname" varchar(255) DEFAULT '',
  "avatar_url" varchar(255) DEFAULT '',
  "bio" text DEFAULT '',
  "updated_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "mobile_phone" varchar(255) NOT NULL,
  PRIMARY KEY ("user_id"),
  CONSTRAINT "username" UNIQUE ("username"),
  CONSTRAINT "phonenumber" UNIQUE ("mobile_phone")
);

ALTER TABLE "department_table" ADD CONSTRAINT "mananger_id" FOREIGN KEY ("manager_id") REFERENCES "employee_table" ("employee_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "department_table" ADD CONSTRAINT "parent_department_id" FOREIGN KEY ("parent_department_id") REFERENCES "department_table" ("department_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "employee_table" ADD CONSTRAINT "department_id" FOREIGN KEY ("department_id") REFERENCES "department_table" ("department_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "employee_table" ADD CONSTRAINT "mananger_id" FOREIGN KEY ("manager_id") REFERENCES "employee_table" ("employee_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "follows" ADD CONSTRAINT "following_user_id" FOREIGN KEY ("following_user_id") REFERENCES "users" ("user_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "follows" ADD CONSTRAINT "followed_user_id" FOREIGN KEY ("followed_user_id") REFERENCES "users" ("user_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "posts" ADD CONSTRAINT "user_id" FOREIGN KEY ("user_id") REFERENCES "users" ("user_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "tasks" ADD CONSTRAINT "user_id" FOREIGN KEY ("user_id") REFERENCES "users" ("user_id") ON DELETE CASCADE ON UPDATE CASCADE;

