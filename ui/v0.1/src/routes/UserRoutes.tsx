import { Route, Routes } from "react-router-dom";
import { UserDetail } from "../components/User/UserDetail";
import { UserList } from "../components/User/UserList";
import { UserNew } from "../components/User/UserNew";

function UserRoutes() {
  return (
    <>
      <div style={{ maxWidth: "100%", minHeight: "100%" }}>
        <Routes>
          <Route path={"list"} element={<UserList />} />
          <Route path={"new"} element={<UserNew />} />
          <Route path={"detail/:user_id"} element={<UserDetail />} />
        </Routes>
      </div>
    </>
  );
}

export default UserRoutes;
