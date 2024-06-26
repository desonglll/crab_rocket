import { Route, Routes } from "react-router-dom";
import { Greet } from "../../components/Common/Greet.tsx";
import EmployeeRoutes from "../../routes/EmployeeRoutes.tsx";
import PostRoutes from "../../routes/PostRoutes.tsx";
import UserRoutes from "../../routes/UserRoutes.tsx";
import TaskRoutes from "../../routes/TaskRoutes.tsx";
import FileRoutes from "../../routes/FileRoutes.tsx";
import RoleRoutes from "../../routes/RoleRoutes.tsx";
import { useState } from "react";
import { Divider, Flex, Layout, MenuTheme } from "antd";
const { Header, Footer, Sider, Content } = Layout;
import { Button } from "antd";
import {
  MenuFoldOutlined,
  MenuUnfoldOutlined,
  SunOutlined,
} from "@ant-design/icons";
import TopMenu from "../../components/Common/TopMenu.tsx";
import SideMenu from "../../components/Common/SideMenu.tsx";
import "./Home.scss";
export function Home() {
  const [collapsed, setCollapsed] = useState(false);
  const [theme, setTheme] = useState<MenuTheme>("light");

  return (
    <>
      <Layout>
        <Flex>
          <Layout>
            <Sider className="sider" collapsed={collapsed} theme={theme}>
              <Button className="demo-logo-vertical"></Button>
              <SideMenu themeMode={theme} />
            </Sider>
          </Layout>

          <Layout
            style={{
              height: "100vh",
              width: "100vw",
              overflow: "auto",
            }}
          >
            <Flex
              vertical={true}
              justify="space-between"
              style={{ height: "100%" }}
            >
              <Flex vertical={true}>
                <Header
                  className="header"
                  style={{
                    position: "sticky",
                    top: 0,
                    zIndex: 1,
                    display: "flex",
                    alignItems: "center",
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
                      height: 64,
                    }}
                  />
                  <Button
                    type="text"
                    onClick={() =>
                      setTheme(theme === "light" ? "dark" : "light")
                    }
                    icon={<SunOutlined />}
                    style={{
                      fontSize: "16px",
                      width: 64,
                      height: 64,
                    }}
                  />
                  <TopMenu themeMode={theme} />
                  <Button className="demo-avatar"></Button>
                </Header>

                <Layout className="layout">
                  <Content className="content">
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
              </Flex>
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
            </Flex>
          </Layout>
        </Flex>
      </Layout>
    </>
  );
}
