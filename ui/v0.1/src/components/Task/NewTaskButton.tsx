import { Button } from "antd";
import { useNavigate } from "react-router-dom";

function NewTaskButton() {
  const navigate = useNavigate();
  const handleNewTask = () => {
    navigate(`/task/new`);
  };
  return (
    <>
      <Button type="primary" onClick={handleNewTask} style={{ margin: 10 }}>
        新建任务
      </Button>
    </>
  );
}

export default NewTaskButton;
