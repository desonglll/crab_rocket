import {Menu, MenuProps, MenuTheme} from "antd";
import menuItems from "../../config/menuItems";
import React, {useEffect, useState} from "react";
import {useLocation, useNavigate} from "react-router-dom";

const SideMenu: React.FC<{ themeMode: MenuTheme }> = ({
  themeMode = "light",
}) => {
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
    <Menu
      defaultSelectedKeys={["1"]}
      defaultOpenKeys={["sub1"]}
      mode="inline"
      items={menuItems}
      selectedKeys={[current]}
      onClick={onClick}
      theme={themeMode}
    />
  );
};

export default SideMenu;
