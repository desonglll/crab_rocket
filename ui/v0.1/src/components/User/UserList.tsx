import { Fade } from "@mui/material";
import { UserTable } from "./UserTable.tsx";
import { BackButton } from "../Common/BackButton.tsx";
import { NewUserButton } from "./NewUserButton.tsx";

export function UserList() {
  return (
    <>
      {
        <Fade in={true}>
          <div>
            <p className="fs-2">User List</p>
            <BackButton />
            <NewUserButton />
            <UserTable />
          </div>
        </Fade>
      }
    </>
  );
}
