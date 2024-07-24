import {Menu, MenuProps, MenuTheme} from "antd";

import React, {useEffect, useState} from "react";
import {useLocation, useNavigate} from "react-router-dom";
import menuItems from "../../config/menuItems";
import "./TopMenu.scss";

const TopMenu: React.FC<{ themeMode: MenuTheme }> = ({
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
    <>
      <Menu
        onClick={onClick}
        selectedKeys={[current]}
        mode="horizontal"
        items={menuItems}
        theme={themeMode}
        style={{ flex: "auto", minWidth: "20px" }}
      />
    </>
  );
};

export default TopMenu;
