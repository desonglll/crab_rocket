import { NavLink } from "react-router-dom";

export function Greet() {
  return (
    <>
      <NavLink to={"/employee/list"}>Go To Employee</NavLink>
    </>
  );
}
