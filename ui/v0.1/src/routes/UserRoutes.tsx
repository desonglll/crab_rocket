import { Route, Routes } from "react-router-dom";
import { UserDetail } from "../components/User/UserDetail";
import { UserList } from "../components/User/UserList";
import { UserNew } from "../components/User/UserNew";
import { TopMenu } from "../components/Common/TopMenu";

function UserRoutes() {
  return (
    <>
      <TopMenu />
      <Routes>
        <Route path={"list"} element={<UserList />} />
        <Route path={"new"} element={<UserNew />} />
        <Route path={"detail/:user_id"} element={<UserDetail />} />
      </Routes>
    </>
  );
}

export default UserRoutes;
