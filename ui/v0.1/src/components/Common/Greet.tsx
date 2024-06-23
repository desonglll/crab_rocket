import { NavLink } from "react-router-dom";

export function Greet() {
  return (
    <>
      <NavLink to={"/employee"}>Go To Employee</NavLink>
    </>
  );
}
