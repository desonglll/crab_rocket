import {Route, Routes} from "react-router-dom";
import {Greet} from "../../components/Common/Greet.tsx";
import EmployeeRoutes from "../../routes/EmployeeRoutes.tsx";
import PostRoutes from "../../routes/PostRoutes.tsx";
import UserRoutes from "../../routes/UserRoutes.tsx";
import TaskRoutes from "../../routes/TaskRoutes.tsx";
import FileRoutes from "../../routes/FileRoutes.tsx";
import RoleRoutes from "../../routes/RoleRoutes.tsx";
import {useEffect, useState} from "react";
import {Button, Divider, Flex, Layout, MenuTheme} from "antd";
import {MenuFoldOutlined, MenuUnfoldOutlined, SunOutlined,} from "@ant-design/icons";
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
  useEffect(() => {
    const handleResize = () => {
      const isMobile = window.matchMedia("(max-width: 767px)").matches;
      setIsMobile(isMobile);
      setCollapsed(isMobile ? true : false);
    };

    // 初始調用以設置狀態
    handleResize();

    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  return (
    <>
      <Flex vertical={false} style={{ height: "100%", width: "100%" }}>
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
                width={"10vw"}
              >
                <Button className="demo-logo-vertical"></Button>
                <SideMenu themeMode={theme} />
              </Sider>
            </Flex>
          </Layout>
        </Flex>

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
              <Header
                style={{ height: "3vh", backgroundColor: "#ffffff" }}
              ></Header>
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

                <Button className="demo-avatar"></Button>
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
