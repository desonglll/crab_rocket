import {Button} from "antd";
import {useNavigate} from "react-router-dom";

export function NewUserButton() {
    const navigate = useNavigate();

    const handleNewUser = () => {
        navigate("/user/user-new");
    };
    return (
        <>
            <Button type="primary" onClick={handleNewUser} style={{margin: 10}}>
                新建用户
            </Button>
        </>
    );
}
