import {Fade} from "@mui/material";
import {Button, Form, Input, message} from "antd";
import TextArea from "antd/es/input/TextArea";
import axios from "axios";
import {useNavigate} from "react-router-dom";
import {SelectUser} from "../Common/SelectUser.tsx";
import {PatchPost} from "../../models/models.ts";

function PostNew() {
    const navigate = useNavigate();
    const [messageApi, contextHolder] = message.useMessage();

    const handleSubmit = async (event: PatchPost) => {
        try {
            if (event.user_id != null) {
                // Send form data to server using axios or fetch
                await axios.post(`post`, event).then((response) => {
                    console.log(response);
                    navigate(-1);
                });
            }
            messageApi.open({
                type: "error",
                content: "請選擇發布用戶",
            });
        } catch (error) {
            console.error("Error submitting form:", error);
        }
    };
    const handleBack = () => {
        navigate(-1); // 返回上一级
    };


    return (
        <>
            {(
                <Fade in={true}>
                    <div>
                        {contextHolder}
                        <Form
                            name="basic"
                            labelCol={{span: 8}}
                            wrapperCol={{span: 16}}
                            style={{maxWidth: 600}}
                            onFinish={handleSubmit}
                            initialValues={{
                                user_id: null,
                            }}
                            autoComplete="off"
                        >
                            <Form.Item>
                                <Button onClick={handleBack}>Back</Button>
                            </Form.Item>
                            <Form.Item
                                label="Title"
                                name="title"
                                rules={[{required: true, message: "請輸入文章的標題！"}]}
                            >
                                <Input/>
                            </Form.Item>
                            <Form.Item label="Body" name="body">
                                <TextArea autoSize/>
                            </Form.Item>
                            <Form.Item
                                label="User Id"
                                name="user_id"
                                rules={[{required: true, message: "請選擇發布的用戶！"}]}
                            >
                                <SelectUser defaultUserId={undefined} onChange={() => {
                                }}/>
                            </Form.Item>
                            <Form.Item label="Status" name="status">
                                <Input/>
                            </Form.Item>
                            <Button type="primary" htmlType="submit">
                                Submit
                            </Button>
                        </Form>
                    </div>
                </Fade>
            )}
        </>
    );
}

export default PostNew;
