CREATE TABLE "department_table" (
  "department_id" serial,
  "department_name" varchar(255) NOT NULL,
  "manager_id" int4,
  "location" varchar(255),
  "creation_date" timestamp,
  "last_update" timestamp DEFAULT CURRENT_TIMESTAMP,
  "description" varchar(255),
  "budget" varchar(255),
  "number_of_employees" int4,
  "parent_department_id" int4,
  "email" varchar(255),
  "phone_number" int4,
  "address" varchar(255),
  "city" varchar(255),
  "state" varchar(255),
  "postal_code" varchar(255),
  PRIMARY KEY ("department_id")
);

CREATE TABLE "employee_table" (
  "employee_id" serial,
  "first_name" varchar(255) NOT NULL,
  "last_name" varchar(255) NOT NULL,
  "gender" char(1),
  "date_of_birth" timestamp,
  "hire_date" timestamp,
  "email" varchar(255),
  "phone_number" int4,
  "department_id" int4,
  "job_title" varchar(255),
  "salary" decimal(10,2),
  "manager_id" int4,
  "address" varchar(255),
  "city" varchar(255),
  "state" varchar(255),
  "postal_code" varchar(255),
  "valid" bool,
  "last_update" timestamp DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY ("employee_id")
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
  PRIMARY KEY ("user_id")
);

ALTER TABLE "department_table" ADD CONSTRAINT "mananger_id" FOREIGN KEY ("manager_id") REFERENCES "employee_table" ("employee_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "department_table" ADD CONSTRAINT "parent_department_id" FOREIGN KEY ("parent_department_id") REFERENCES "department_table" ("department_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "employee_table" ADD CONSTRAINT "department_id" FOREIGN KEY ("department_id") REFERENCES "department_table" ("department_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "employee_table" ADD CONSTRAINT "mananger_id" FOREIGN KEY ("manager_id") REFERENCES "employee_table" ("employee_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "follows" ADD CONSTRAINT "following_user_id" FOREIGN KEY ("following_user_id") REFERENCES "users" ("user_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "follows" ADD CONSTRAINT "followed_user_id" FOREIGN KEY ("followed_user_id") REFERENCES "users" ("user_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "posts" ADD CONSTRAINT "user_id" FOREIGN KEY ("user_id") REFERENCES "users" ("user_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "tasks" ADD CONSTRAINT "user_id" FOREIGN KEY ("user_id") REFERENCES "users" ("user_id") ON DELETE CASCADE ON UPDATE CASCADE;

