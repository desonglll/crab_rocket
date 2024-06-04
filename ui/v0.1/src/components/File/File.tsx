import {Route, Routes} from "react-router-dom";
import {FileTable} from "./FileTable.tsx";

export function File() {
    return (
        <>
            <Routes>
                <Route path={""} element={<FileTable/>}/>
            </Routes>
        </>
    );
}