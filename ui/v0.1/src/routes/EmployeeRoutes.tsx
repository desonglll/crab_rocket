import { Route, Routes } from "react-router-dom";
import { EmployeeList } from "../components/Employee/EmployeeList";
import { TopMenu } from "../components/Common/TopMenu";
import { EmployeeDetail } from "../components/Employee/EmployeeDetail";

function EmployeeRoutes() {
  return (
    <>
      <TopMenu />
      <Routes>
        <Route path="/" Component={EmployeeList} />
        <Route path={"detail/:employee_id"} element={<EmployeeDetail />} />
      </Routes>
    </>
  );
}

export default EmployeeRoutes;
