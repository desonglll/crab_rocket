import TopMenu from "./components/TopMenu";

function App() {
  return (
    <>
      <div
        style={{
          display: "flex",
          width: "100%",
          height: "100%",
          flexDirection: "row",
        }}
      >
        <div style={{ backgroundColor: "blue", width: "20%" }}>
          <p>sider</p>
        </div>
        <div style={{ display: "flex", width: "80%", flexDirection: "column" }}>
          <div style={{ backgroundColor: "yellow", height: "10%" }}>
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
