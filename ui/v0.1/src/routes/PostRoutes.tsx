import {Route, Routes} from "react-router-dom";
import PostDetail from "../components/Post/PostDetail";
import PostList from "../components/Post/PostList";
import PostNew from "../components/Post/PostNew";

function PostRoutes() {
  return (
    <>
      <Routes>
        <Route path={"list"} element={<PostList />} />
        <Route path={"new"} element={<PostNew />} />
        <Route path={"detail/:post_id"} element={<PostDetail />} />
      </Routes>
    </>
  );
}

export default PostRoutes;
