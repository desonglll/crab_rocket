import { useEffect, useRef, useState } from "react";
import TopMenu from "./components/TopMenu";
import "./App.css";

function App() {
  const [width, setWidth] = useState(300); // 初始宽度为300px
  const [isResizing, setIsResizing] = useState(false);

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
      <div
        style={{
          display: "flex",
          width: "100%",
          height: "100%",
          flexDirection: "row",
        }}
        onMouseMove={handleMouseMove}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseUp} // 鼠标移出容器时停止调整
      >
        <div style={{ backgroundColor: "blue", width: `${width}px` }}>
          <p>sider</p>
        </div>
        <div className="resize-handle" onMouseDown={handleMouseDown} />

        <div style={{ display: "flex", flexDirection: "column", flex: 1 }}>
          <div
            style={{ backgroundColor: "yellow", height: "10%", width: "100%" }}
          >
            <TopMenu />
          </div>
          <div
            style={{
              backgroundColor: "red",
              height: "90%",
              overflow: "scroll",
            }}
          >
            <div style={{ height: "1000px", width: "100%" }}>aaa</div>
          </div>
        </div>
      </div>
    </>
  );
}

export default App;
