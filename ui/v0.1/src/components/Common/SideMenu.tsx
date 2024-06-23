import { Menu, MenuProps } from "antd";
import menuItems from "../../config/menuItems";
import { useState, useEffect } from "react";
import { useNavigate, useLocation } from "react-router-dom";
function SideMenu() {
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
    <div>
      <Menu
        defaultSelectedKeys={["1"]}
        defaultOpenKeys={["sub1"]}
        mode="vertical"
        items={menuItems}
        selectedKeys={[current]}
        onClick={onClick}
        // style={{ background: "#ffffff" }}
      />
    </div>
  );
}

export default SideMenu;
