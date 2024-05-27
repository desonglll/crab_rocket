-- Your SQL goes here
CREATE TABLE "follows" (
  "following_user_id" int4 NOT NULL DEFAULT -1,
  "followed_user_id" int4 NOT NULL DEFAULT -1,
  "created_at" timestamp DEFAULT CURRENT_TIMESTAMP,
  "follow_id" serial,
  PRIMARY KEY ("follow_id")
);


-- 向 follows 表插入示例数据
INSERT INTO follows (following_user_id, followed_user_id, created_at)
VALUES
(1, 2, '2023-01-01'),
(2, 3, '2023-02-01'),
(3, 1, '2023-03-01');

ALTER TABLE "follows" ADD CONSTRAINT "following_user_id" FOREIGN KEY ("following_user_id") REFERENCES "user_table" ("user_id") ON DELETE CASCADE ON UPDATE CASCADE;
ALTER TABLE "follows" ADD CONSTRAINT "followed_user_id" FOREIGN KEY ("followed_user_id") REFERENCES "user_table" ("user_id") ON DELETE CASCADE ON UPDATE CASCADE;
