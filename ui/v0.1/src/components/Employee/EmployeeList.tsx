import {Collapse, CollapseProps} from "antd";
import {BackButton} from "../Common/BackButton.tsx";
import EmployeeTable from "./EmployeeTable.tsx";
import NewEmployeeButton from "./NewEmployeeButton.tsx";

export function EmployeeList() {
  const items: CollapseProps["items"] = [
    {
      key: "1",
      label: "员工列表",
      children: <EmployeeTable />,
    },
  ];
  return (
    <>
      <p className="fs-2">Employee List</p>
      <BackButton />
      <NewEmployeeButton />
      <Collapse items={items} defaultActiveKey={["1"]} />
    </>
  );
}
