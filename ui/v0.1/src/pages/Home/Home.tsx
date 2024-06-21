import { Route, Routes } from "react-router-dom";
import { Greet } from "../../components/Common/Greet.tsx";
import EmployeeRoutes from "../../routes/EmployeeRoutes.tsx";
import PostRoutes from "../../routes/PostRoutes.tsx";
import UserRoutes from "../../routes/UserRoutes.tsx";
import TaskRoutes from "../../routes/TaskRoutes.tsx";
import FileRoutes from "../../routes/FileRoutes.tsx";
import RoleRoutes from "../../routes/RoleRoutes.tsx";

export function Home() {
  return (
    <>
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
    </>
  );
}
