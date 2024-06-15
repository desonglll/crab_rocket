-- Your SQL goes here
CREATE TABLE "user_table" (
  "user_id" serial,
  "username" varchar(255) NOT NULL,
  "role_id" int4 DEFAULT -1,
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

-- 向 users 表插入示例数据
INSERT INTO user_table (username, role_id, email, password, fullname, avatar_url, bio, mobile_phone)
VALUES
('john_doe', 1, 'john.doe@example.com', 'password1', 'John Doe', 'https://example.com/avatar1.jpg', 'Hello, I am John Doe', '123-456-7890'),
('jane_smith', 2, 'jane.smith@example.com', 'password2', 'Jane Smith', 'https://example.com/avatar2.jpg', 'Hi, I am Jane Smith', '987-654-3210'),
('mike_johnson', 3, 'mike.johnson@example.com', 'password3', 'Mike Johnson', 'https://example.com/avatar3.jpg', 'Nice to meet you!', '111-222-3333');

ALTER TABLE "user_table" ADD CONSTRAINT "role_id" FOREIGN KEY ("role_id") REFERENCES "role_table" ("role_id") ON DELETE CASCADE ON UPDATE CASCADE;
