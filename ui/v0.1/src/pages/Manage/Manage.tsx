import { Route, Routes } from "react-router-dom";
import { Role } from "../../components/Role/Role.tsx";
import { User } from "../../components/User/User.tsx";

export const Manage = () => {
	return (
		<>
			<Routes>
				<Route path={"role/*"} element={<Role />} />
				<Route path={"user/*"} element={<User />} />
			</Routes>
		</>
	);
};
