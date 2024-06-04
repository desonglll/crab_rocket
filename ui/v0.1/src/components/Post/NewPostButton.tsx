import {Button} from "antd";
import {useNavigate} from "react-router-dom";

export function NewPostButton() {
    const navigate = useNavigate();

    const handleNewPost = () => {
        navigate("/post/post-new");
    };
    return (
        <>
            <Button type="primary" onClick={handleNewPost} style={{margin: 10}}>
                新建文章
            </Button>
        </>
    );
}
