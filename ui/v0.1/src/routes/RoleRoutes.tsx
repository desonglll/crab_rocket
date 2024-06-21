import { Routes, Route } from "react-router-dom";
import { RoleDetail } from "../components/Role/RoleDetail";
import { RoleNew } from "../components/Role/RoleNew";
import { RoleTable } from "../components/Role/RoleTable";
import { TopMenu } from "../components/Common/TopMenu";

function RoleRoutes() {
  return (
    <div>
      <TopMenu />
      <Routes>
        <Route path={"list"} element={<RoleTable />} />
        <Route path={"new"} element={<RoleNew />} />
        <Route path={"detail/:role_id"} element={<RoleDetail />} />
      </Routes>
    </div>
  );
}

export default RoleRoutes;
