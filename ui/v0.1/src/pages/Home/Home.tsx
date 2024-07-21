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
      <Flex style={{ height: "100%", width: "100%" }}>
        <Layout>
          <Flex vertical={false} style={{ height: "100%", width: "100%" }}>
            <Flex style={{ height: "100%", width: "auto" }}>
              <Layout
                style={{
                  overflow: "auto",
                }}
              >
                <Flex>
                  <Sider className="sider" collapsed={collapsed} theme={theme}>
                    <Button className="demo-logo-vertical"></Button>
                    <SideMenu themeMode={theme} />
                  </Sider>
                </Flex>
              </Layout>
            </Flex>

            <Flex style={{ width: "100%", height: "100%" }}>
              <Layout>
                <Flex
                  vertical={true}
                  justify="space-between"
                  style={{ height: "100%", width: "100%" }}
                >
                  <Header
                    className="header"
                    style={{
                      position: "sticky",
                      top: 0,
                      zIndex: 1,
                      display: "flex",
                      alignItems: "center",
                      width: "100%",
                    }}
                  >
                    <Button
                      type="text"
                      icon={
                        collapsed ? (
                          <MenuUnfoldOutlined />
                        ) : (
                          <MenuFoldOutlined />
                        )
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
                    <div style={{ display: "flex" }}>
                      <TopMenu themeMode={theme} />
                    </div>

                    <Button className="demo-avatar"></Button>
                  </Header>

                  <Layout
                    className="layout"
                    style={{
                      width: "100%",
                      height: "100%",
                      overflow: "scroll",
                    }}
                  >
                    <Content className="content">
                      <div style={{}}>
                        <Routes>
                          <Route path={""} element={<Greet />} />
                          <Route path={"greet"} element={<Greet />} />
                          <Route path={"post/*"} element={<PostRoutes />} />
                          <Route path={"task/*"} element={<TaskRoutes />} />
                          <Route
                            path={"employee/*"}
                            element={<EmployeeRoutes />}
                          />
                          <Route path={"file/*"} element={<FileRoutes />} />
                          <Route path={"user/*"} element={<UserRoutes />} />
                          <Route path={"role/*"} element={<RoleRoutes />} />
                        </Routes>
                      </div>
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
                </Flex>
              </Layout>
            </Flex>
          </Flex>
        </Layout>
      </Flex>
    </>
  );
}
