import { Fade } from "@mui/material";
import { TaskTable } from "./TaskTable.tsx";
import { BackButton } from "../Common/BackButton.tsx";
import NewTaskButton from "./NewTaskButton.tsx";

function TaskList() {
  return (
    <>
      {
        <Fade in={true}>
          <div>
            <p className="fs-2">Task List</p>
            <BackButton />
            <NewTaskButton />
            <TaskTable />
          </div>
        </Fade>
      }
    </>
  );
}

export default TaskList;
