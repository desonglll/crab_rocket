import { BackButton } from "../Common/BackButton.tsx";
import EmployeeTable from "./EmployeeTable.tsx";
import NewEmployeeButton from "./NewEmployeeButton.tsx";

export function EmployeeList() {
  return (
    <>
      <p className="fs-2">Employee List</p>
      <BackButton />
      <NewEmployeeButton />
      <EmployeeTable />
    </>
  );
}
