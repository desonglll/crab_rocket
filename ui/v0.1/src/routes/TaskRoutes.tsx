import {Route, Routes} from "react-router-dom";
import TaskDetail from "../components/Task/TaskDetail";
import TaskList from "../components/Task/TaskList";
import TaskNew from "../components/Task/TaskNew";

function TaskRoutes() {
  return (
    <>
      <Routes>
        <Route path={"list"} element={<TaskList />} />
        <Route path={"new"} element={<TaskNew />} />
        <Route path={"detail/:task_id"} element={<TaskDetail />} />
      </Routes>
    </>
  );
}

export default TaskRoutes;
