import TaskList from "./TaskList.tsx";
import {Route, Routes} from "react-router-dom";
import TaskNew from "./TaskNew.tsx";
import TaskDetail from "./TaskDetail.tsx";

export function Task() {
    return (
        <>
            <Routes>
                <Route path={"list"} element={<TaskList/>}/>
                <Route path={"new"} element={<TaskNew/>}/>
                <Route path={"detail/:task_id"} element={<TaskDetail/>}/>
            </Routes>
        </>
    );
}