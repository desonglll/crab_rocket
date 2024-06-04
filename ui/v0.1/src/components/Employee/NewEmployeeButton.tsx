import { Button } from "antd";
import { useNavigate } from "react-router-dom";

function NewEmployeeButton() {
  const navigate = useNavigate();
  const handleNewEmployee = () => {
    navigate(`/employee/new`);
  };
  return (
    <>
      <Button type="primary" onClick={handleNewEmployee} style={{ margin: 10 }}>
        新建员工
      </Button>
    </>
  );
}

export default NewEmployeeButton;
