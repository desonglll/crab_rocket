import { Route, Routes } from "react-router-dom";
import { Greet } from "../../components/Common/Greet.tsx";
import EmployeeRoutes from "../../routes/EmployeeRoutes.tsx";
import PostRoutes from "../../routes/PostRoutes.tsx";
import UserRoutes from "../../routes/UserRoutes.tsx";
import TaskRoutes from "../../routes/TaskRoutes.tsx";
import FileRoutes from "../../routes/FileRoutes.tsx";
import RoleRoutes from "../../routes/RoleRoutes.tsx";
import { useState } from "react";
import { Layout } from "antd";
const { Header, Footer, Sider, Content } = Layout;
import { Button } from "antd";
import { MenuFoldOutlined, MenuUnfoldOutlined } from "@ant-design/icons";
import { TopMenu } from "../../components/Common/TopMenu.tsx";
import SideMenu from "../../components/Common/SideMenu.tsx";

import "./Home.scss";
export function Home() {
  const [collapsed, setCollapsed] = useState(false);

  const toggleCollapsed = () => {
    setCollapsed(!collapsed);
  };
  return (
    <>
      <Layout style={{ height: "100%" }}>
        <Header
          className="header"
          style={{
            position: "sticky",
            top: 0,
            zIndex: 1,
            // width: "100%",
            display: "flex",
            alignItems: "center",
          }}
        >
          <TopMenu />
        </Header>
        <Layout className="layout">
          <Sider className="sider" width="10%" collapsed={collapsed}>
            <div style={{ display: "flex", justifyContent: "flex-end" }}>
              <Button
                type="primary"
                onClick={toggleCollapsed}
                style={{ marginBottom: 16 }}
              >
                {collapsed ? <MenuUnfoldOutlined /> : <MenuFoldOutlined />}
              </Button>
            </div>
            <SideMenu />
          </Sider>
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
        <Footer className="footer">Footer</Footer>
      </Layout>
    </>
  );
}
