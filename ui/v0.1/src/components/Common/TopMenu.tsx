import { Menu, MenuProps } from "antd";

import { useEffect, useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import menuItems from "../../config/menuItems";

export function TopMenu() {
  const [current, setCurrent] = useState("");
  const navigate = useNavigate();
  const location = useLocation();
  useEffect(() => {
    // 根据当前的 URL 设置初始值
    const path = location.pathname;

    if (path) {
      setCurrent(path);
    }
  }, [location]);

  const onClick: MenuProps["onClick"] = (e) => {
    setCurrent(e.key);
    console.log(e.key);

    navigate(`${e.key}`);
  };
  return (
    <div style={{ width: "100%" }}>
      <Menu
        onClick={onClick}
        selectedKeys={[current]}
        mode="horizontal"
        items={menuItems}
      />
    </div>
  );
}
