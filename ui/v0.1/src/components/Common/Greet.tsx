import { NavLink } from "react-router-dom";
import { TopMenu } from "./TopMenu";

export function Greet() {
  return (
    <>
      <TopMenu />
      <NavLink to={"/employee"}>Go To Employee</NavLink>
    </>
  );
}
