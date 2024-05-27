-- Your SQL goes here

CREATE TABLE "tasks" (
  "id" serial,
  "title" text NOT NULL,
  "content" text DEFAULT '',
  "created_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "updated_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "user_id" int4 DEFAULT -1,
  PRIMARY KEY ("id")
);

-- 向 tasks 表插入示例数据
INSERT INTO tasks (title, content, user_id, created_at, updated_at)
VALUES
('Task 1', 'Task 1 content', 1, '2023-01-01', '2023-01-01'),
('Task 2', 'Task 2 content', 2, '2023-02-01', '2023-02-01'),
('Task 3', 'Task 3 content', 3, '2023-03-01', '2023-03-01');

ALTER TABLE "tasks" ADD CONSTRAINT "user_id" FOREIGN KEY ("user_id") REFERENCES "user_table" ("user_id") ON DELETE CASCADE ON UPDATE CASCADE;
