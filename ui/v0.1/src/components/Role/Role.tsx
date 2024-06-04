import {Route, Routes} from "react-router-dom";
import {RoleNew} from "./RoleNew.tsx";
import {RoleDetail} from "./RoleDetail.tsx";
import {RoleTable} from "./RoleTable.tsx";

export function Role() {
    return (
        <>
            <Routes>
                <Route path={"role-list"} element={<RoleTable/>}/>
                <Route path={"role-new"} element={<RoleNew/>}/>
                <Route path={"role-detail/:role_id"} element={<RoleDetail/>}/>
            </Routes>
        </>
    );
}