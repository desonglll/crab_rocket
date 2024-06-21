import { Routes, Route } from "react-router-dom";
import { FileTable } from "../components/File/FileTable";
import { TopMenu } from "../components/Common/TopMenu";

function FileRoutes() {
  return (
    <>
      <TopMenu />
      <Routes>
        <Route path={""} element={<FileTable />} />
      </Routes>
    </>
  );
}

export default FileRoutes;
