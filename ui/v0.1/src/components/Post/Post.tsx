import {Route, Routes} from "react-router-dom";
import PostNew from "./PostNew.tsx";
import PostDetail from "./PostDetail.tsx";
import PostList from "./PostList.tsx";

export function Post() {
    return (
        <>
            <Routes>
                <Route path={"post-list"} element={<PostList/>}/>
                <Route path={"post-new"} element={<PostNew/>}/>
                <Route path={"post-detail/:post_id"} element={<PostDetail/>}/>
            </Routes>
        </>
    );
}