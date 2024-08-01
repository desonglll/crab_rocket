import { Route, Routes } from "react-router-dom";
import { Greet } from "../../components/Common/Greet.tsx";
import EmployeeRoutes from "../../routes/EmployeeRoutes.tsx";
import PostRoutes from "../../routes/PostRoutes.tsx";
import UserRoutes from "../../routes/UserRoutes.tsx";
import TaskRoutes from "../../routes/TaskRoutes.tsx";
import FileRoutes from "../../routes/FileRoutes.tsx";
import RoleRoutes from "../../routes/RoleRoutes.tsx";
import { useEffect, useState } from "react";
import { Button, Divider, Flex, Layout, type MenuTheme } from "antd";
import {
  MenuFoldOutlined,
  MenuUnfoldOutlined,
  SunOutlined,
} from "@ant-design/icons";
import TopMenu from "../../components/Common/TopMenu.tsx";
import SideMenu from "../../components/Common/SideMenu.tsx";
import "./Home.scss";

const { Header, Footer, Sider, Content } = Layout;

export function Home() {
  const [isMobile, setIsMobile] = useState(
    window.matchMedia("(max-width: 767px)").matches
  );
  const [collapsed, setCollapsed] = useState(false);
  const [theme, setTheme] = useState<MenuTheme>("light");
  const [width, setWidth] = useState(300); // 初始宽度为300px
  const [isResizing, setIsResizing] = useState(false);

  useEffect(() => {
    const handleResize = () => {
      const isMobile = window.matchMedia("(max-width: 767px)").matches;
      setIsMobile(isMobile);
      setCollapsed(isMobile);
    };

    // 初始調用以設置狀態
    handleResize();

    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);
  const handleMouseDown = () => {
    document.body.style.userSelect = "none"; // 禁止文本选择
    document.body.style.cursor = "ew-resize"; // 改变鼠标样式
    setIsResizing(true);
  };

  const handleMouseMove = (e) => {
    if (isResizing) {
      setWidth(e.clientX);
    }
  };

  const handleMouseUp = () => {
    setIsResizing(false);
    document.body.style.userSelect = ""; // 重新启用文本选择
    document.body.style.cursor = ""; // 恢复默认鼠标样式
  };
  return (
    <>
      <Flex
        vertical={false}
        style={{ height: "100%", width: "100%" }}
        onMouseMove={handleMouseMove}
        onMouseUp={handleMouseUp}
      >
        <Flex style={{ height: "100%", width: "auto" }}>
          <Layout
            style={{
              overflow: "auto",
              width: "100%",
            }}
          >
            <Flex>
              <Sider
                className="sider"
                collapsed={collapsed}
                theme={theme}
                width={width}
              >
                <Button className="demo-logo-vertical" />
                <SideMenu themeMode={theme} />
              </Sider>
            </Flex>
          </Layout>
        </Flex>
        <div className="resize-handle" onMouseDown={handleMouseDown} />
        <Flex
          style={{
            width: "auto",
            height: "100%",
            minWidth: "20px",
            flex: "auto",
          }}
        >
          <Layout style={{ minWidth: "20px", width: "100%" }}>
            {isMobile ? (
              <Header style={{ height: "3vh", backgroundColor: "#ffffff" }} />
            ) : (
              <Header
                className="header"
                style={{
                  display: "flex",
                  alignItems: "center",
                  width: "100%",
                }}
              >
                <Button
                  type="text"
                  icon={
                    collapsed ? <MenuUnfoldOutlined /> : <MenuFoldOutlined />
                  }
                  onClick={() => setCollapsed(!collapsed)}
                  style={{
                    fontSize: "16px",
                    width: 64,
                    height: 30,
                  }}
                  className="hide-on-mobile"
                />
                <Button
                  type="text"
                  onClick={() => setTheme(theme === "light" ? "dark" : "light")}
                  icon={<SunOutlined />}
                  style={{
                    fontSize: "16px",
                    width: 64,
                    height: 30,
                  }}
                  className="hide-on-mobile"
                />

                <TopMenu themeMode={theme} />

                <Button className="demo-avatar" />
              </Header>
            )}

            <Layout
              className="layout"
              style={{
                width: "100%",
                height: "100%",
              }}
            >
              <Content
                className="content"
                style={{
                  width: "100%",
                  height: "auto",
                  overflow: "scroll",
                }}
              >
                <Routes>
                  <Route path={""} element={<Greet />} />
                  <Route path={"greet"} element={<Greet />} />
                  <Route path={"post/*"} element={<PostRoutes />} />
                  <Route path={"task/*"} element={<TaskRoutes />} />
                  <Route path={"employee/*"} element={<EmployeeRoutes />} />
                  <Route path={"file/*"} element={<FileRoutes />} />
                  <Route path={"user/*"} element={<UserRoutes />} />
                  <Route path={"role/*"} element={<RoleRoutes />} />
                </Routes>
              </Content>
            </Layout>

            <Divider />
            <Footer
              className="footer"
              style={{
                position: "sticky",
                bottom: 0,
                zIndex: 1,
                display: "flex",
                alignItems: "center",
              }}
            >
              2023-2024 © Copyright Alright Received
            </Footer>
          </Layout>
        </Flex>
      </Flex>
    </>
  );
}
