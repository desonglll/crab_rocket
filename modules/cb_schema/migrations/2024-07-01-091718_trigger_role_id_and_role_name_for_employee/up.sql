-- Your SQL goes here
-- 创建函数
CREATE OR REPLACE FUNCTION update_role_id()
RETURNS TRIGGER AS $$
BEGIN
  -- 查找 role_name 对应的 role_id
  SELECT role_id INTO NEW.role_id FROM role_table WHERE role_name = NEW.role_name;
  
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 创建触发器，在插入或更新时调用函数
CREATE TRIGGER set_role_id
BEFORE INSERT OR UPDATE ON employee_table
FOR EACH ROW EXECUTE FUNCTION update_role_id();
