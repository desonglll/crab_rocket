import "bootstrap/dist/css/bootstrap.css";
import { Fade } from "@mui/material";
import { BackButton } from "../Common/BackButton.tsx";
import { NewPostButton } from "./NewPostButton.tsx";
import { PostTable } from "./PostTable.tsx";
import Collapse, { type CollapseProps } from "antd/es/collapse/Collapse";

function PostList() {
  const items: CollapseProps["items"] = [
    {
      key: "1",
      label: "文章列表",
      children: <PostTable />,
    },
  ];
  return (
    <>
      {
        <Fade in={true}>
          <div>
            <p className="fs-2">文章列表</p>
            <BackButton />
            <NewPostButton />
            <Collapse items={items} defaultActiveKey={["1"]} />
          </div>
        </Fade>
      }
    </>
  );
}

export default PostList;
