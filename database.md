## Trigger

```sql
CREATE OR REPLACE FUNCTION fill_username()
RETURNS TRIGGER AS $$
BEGIN
  -- 通过查询 users 表获取对应的用户名
  SELECT username INTO NEW.username FROM users WHERE user_id = NEW.user_id;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER fill_username_trigger
BEFORE INSERT ON posts
FOR EACH ROW EXECUTE FUNCTION fill_username();
```
