import {TopMenu} from "../../components/Common/TopMenu.tsx";
import {Route, Routes} from "react-router-dom";
import {Post} from "../../components/Post/Post.tsx";
import {Task} from "../../components/Task/Task.tsx";
import Employee from "../../components/Employee/Employee.tsx";
import {Greet} from "../../components/Common/Greet.tsx";
import {File} from "../../components/File/File.tsx";
import {Manage} from "../Manage/Manage.tsx";

export function Home() {
    return (
        <>
            <TopMenu/>
            <Routes>
                <Route path={"greet"} element={<Greet/>}/>
                <Route path={"post/*"} element={<Post/>}/>
                <Route path={"task/*"} element={<Task/>}/>
                <Route path={"employee/employee-list"} element={<Employee/>}/>
                <Route path={"file/*"} element={<File/>}/>
                <Route path={'manage/*'} element={<Manage/>}/>
            </Routes>
        </>
    )
}