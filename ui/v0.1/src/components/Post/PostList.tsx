import "bootstrap/dist/css/bootstrap.css";
import { Fade } from "@mui/material";
import { BackButton } from "../Common/BackButton.tsx";
import { NewPostButton } from "./NewPostButton.tsx";
import { PostTable } from "./PostTable.tsx";

function PostList() {
  return (
    <>
      {
        <Fade in={true}>
          <div>
            <p className="fs-2">文章列表</p>
            <BackButton />
            <NewPostButton />
            <PostTable />
          </div>
        </Fade>
      }
    </>
  );
}

export default PostList;
