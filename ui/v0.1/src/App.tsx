import { Route, BrowserRouter, Routes } from "react-router-dom";
import { Home } from "./pages/Home/Home.tsx";
import { ConfigProvider } from "antd";
import zhCN from "antd/es/locale/zh_CN";

function App() {
  return (
    <>
      <ConfigProvider
        theme={{
          token: {
            paddingLG: 10,
          },
        }}
        locale={zhCN}
      >
        <BrowserRouter>
          <Routes>
            <Route path={"/*"} element={<Home />} />
          </Routes>
        </BrowserRouter>
      </ConfigProvider>
    </>
  );
}

export default App;
