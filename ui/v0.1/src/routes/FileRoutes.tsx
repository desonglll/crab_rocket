import { Routes, Route } from "react-router-dom";
import { FileTable } from "../components/File/FileTable";
import FileList from "../components/File/FileList";

function FileRoutes() {
  return (
    <>
      <Routes>
        <Route path={""} element={<FileTable />} />
        <Route path={"list"} element={<FileList />} />
      </Routes>
    </>
  );
}

export default FileRoutes;
