import {Fade} from "@mui/material";
import {Button, Form, Input,} from "antd";
import axios from "axios";
import {useEffect,} from "react";
import {useNavigate} from "react-router-dom";
import {SelectUser} from "../Common/SelectUser.tsx";
import {PatchTask} from "../../models/models.ts";

function TaskNew() {
    const navigate = useNavigate();
    const handleSubmit = async (event: PatchTask) => {
        console.log(event);
        try {
            // Send form data to server using axios or fetch
            await axios.post(`task`, event).then(() => {
                navigate(-1);
            });
        } catch (error) {
            console.error("Error submitting form:", error);
        }
    };
    const handleBack = () => {
        navigate(-1);
    };

    useEffect(() => {
    }, []);
    return (
        <>
            {(
                <Fade in={true}>
                    <div>
                        <Form
                            name="basic"
                            labelCol={{span: 8}}
                            wrapperCol={{span: 16}}
                            style={{maxWidth: 600}}
                            onFinish={handleSubmit}
                            initialValues={{
                                user_id: 1,
                            }}
                            autoComplete="off"
                        >
                            <Form.Item>
                                <Button onClick={handleBack}>Back</Button>
                            </Form.Item>
                            <Form.Item label={"Title"} name={"title"}>
                                <Input/>
                            </Form.Item>
                            <Form.Item label={"Content"} name={"content"}>
                                <Input/>
                            </Form.Item>
                            <Form.Item label={"User"} name={"user_id"}>
                                <SelectUser defaultUserId={undefined} onChange={() => {
                                }}/>
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

export default TaskNew;
