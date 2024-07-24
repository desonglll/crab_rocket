import {Route, Routes} from "react-router-dom";
import {EmployeeList} from "../components/Employee/EmployeeList";
import {EmployeeDetail} from "../components/Employee/EmployeeDetail";
import EmployeeNew from "../components/Employee/EmployeeNew";

function EmployeeRoutes() {
  return (
    <>
      <Routes>
        <Route path="list" Component={EmployeeList} />
        <Route path="new" Component={EmployeeNew} />
        <Route path={"detail/:employee_id"} element={<EmployeeDetail />} />
      </Routes>
    </>
  );
}

export default EmployeeRoutes;
