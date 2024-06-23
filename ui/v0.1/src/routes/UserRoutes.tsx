import { Route, Routes } from "react-router-dom";
import { UserDetail } from "../components/User/UserDetail";
import { UserList } from "../components/User/UserList";
import { UserNew } from "../components/User/UserNew";

function UserRoutes() {
  return (
    <>
      <Routes>
        <Route path={"list"} element={<UserList />} />
        <Route path={"new"} element={<UserNew />} />
        <Route path={"detail/:user_id"} element={<UserDetail />} />
      </Routes>
    </>
  );
}

export default UserRoutes;
