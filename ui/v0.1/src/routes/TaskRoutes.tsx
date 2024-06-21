import { Routes, Route } from "react-router-dom";
import TaskDetail from "../components/Task/TaskDetail";
import TaskList from "../components/Task/TaskList";
import TaskNew from "../components/Task/TaskNew";
import { TopMenu } from "../components/Common/TopMenu";

function TaskRoutes() {
  return (
    <>
      <TopMenu />
      <Routes>
        <Route path={"list"} element={<TaskList />} />
        <Route path={"new"} element={<TaskNew />} />
        <Route path={"detail/:task_id"} element={<TaskDetail />} />
      </Routes>
    </>
  );
}

export default TaskRoutes;
