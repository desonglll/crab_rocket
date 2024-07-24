import {Route, Routes} from "react-router-dom";
import {FileTable} from "../components/File/FileTable";
import FileList from "../components/File/FileList";
import FileUpload from "../components/File/FileUpload";

function FileRoutes() {
  return (
    <>
      <Routes>
        <Route path={""} element={<FileTable />} />
        <Route path={"list"} element={<FileList />} />
        <Route path={"upload"} element={<FileUpload />} />
      </Routes>
    </>
  );
}

export default FileRoutes;
