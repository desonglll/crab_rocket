import {Route, Routes} from "react-router-dom";
import {RoleDetail} from "../components/Role/RoleDetail";
import {RoleNew} from "../components/Role/RoleNew";
import {RoleTable} from "../components/Role/RoleTable";

function RoleRoutes() {
  return (
    <>
      <Routes>
        <Route path={"list"} element={<RoleTable />} />
        <Route path={"new"} element={<RoleNew />} />
        <Route path={"detail/:role_id"} element={<RoleDetail />} />
      </Routes>
    </>
  );
}

export default RoleRoutes;
