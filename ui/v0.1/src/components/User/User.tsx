import { UserList } from "./UserList.tsx";
import { Route, Routes } from "react-router-dom";
import { UserNew } from "./UserNew.tsx";
import { UserDetail } from "./UserDetail.tsx";

export function User() {
	return (
		<>
			<Routes>
				<Route path={"user-list"} element={<UserList />} />
				<Route path={"user-new"} element={<UserNew />} />
				<Route path={"user-detail/:user_id"} element={<UserDetail />} />
			</Routes>
		</>
	);
}
