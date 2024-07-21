import { Route, BrowserRouter, Routes } from "react-router-dom";
import { Home } from "./pages/Home/Home.tsx";
import { ConfigProvider } from "antd";
import zhCN from "antd/es/locale/zh_CN";
import "antd/dist/reset.css";
import "./index.css";

function App() {
  const items = [];

  for (let i = 0; i < 10; i += 1) {
    items.push({
      label: `Item ${i}`,
      key: i,
    });
  }

  return (
    <>
      <ConfigProvider
        theme={{
          token: {
            paddingLG: 10,
          },
          components: {
            Menu: {
              groupTitleLineHeight: "1.0",
              itemHeight: 35,
              activeBarHeight: 2,
              // collapsedIconSize: 10,
            },
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
