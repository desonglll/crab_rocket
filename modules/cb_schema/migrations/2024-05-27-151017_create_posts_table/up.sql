-- Your SQL goes here

CREATE TABLE "posts" (
  "post_id" serial,
  "title" varchar(255) DEFAULT 'Untitled',
  "body" text DEFAULT 'No Content',
  "user_id" int4 DEFAULT -1,
  "status" varchar(255) DEFAULT 'unknown status',
  "created_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "updated_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "username" varchar(255),
  PRIMARY KEY ("post_id")
);

-- 向 posts 表插入示例数据
INSERT INTO posts (title, body, user_id, status, username, created_at, updated_at)
VALUES
('Post 1', 'This is post 1', 1, 'active', 'john_doe', '2023-01-01', '2023-01-01'),
('Post 2', 'This is post 2', 2, 'active', 'jane_smith', '2023-02-01', '2023-02-01'),
('Post 3', 'This is post 3', 3, 'active', 'mike_johnson', '2023-03-01', '2023-03-01');

ALTER TABLE "posts" ADD CONSTRAINT "user_id" FOREIGN KEY ("user_id") REFERENCES "user_table" ("user_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "posts" ADD CONSTRAINT "username" FOREIGN KEY ("username") REFERENCES "user_table" ("username");
