import {Fade} from "@mui/material";
import {Button, Form, Input, message} from "antd";
import axios from "axios";
import {useEffect, useState} from "react";
import {useNavigate, useParams} from "react-router-dom";
import {SelectUser} from "../Common/SelectUser.tsx";
import dayjs from "dayjs";
import TextArea from "antd/es/input/TextArea";
import {PatchTask, Task} from "../../models/models.ts";

function TaskDetail() {
    const {task_id} = useParams();
    const [task, setTask] = useState<Task>();
    const [loading, setLoading] = useState(true);
    const [messageApi, contextHolder] = message.useMessage();

    const navigate = useNavigate();
    useEffect(() => {
        const fetchTask = async () => {
            try {
                const response = await axios.get(`task/${task_id}`);
                setTask(response.data.data);
            } catch (error) {
                console.log(error);
            }
        };
        fetchTask().then(() => {
            setLoading(!loading);
        });
    }, []);
    const onFinish = (data: PatchTask) => {
        try {
            axios.patch(`task/${task_id}`, data).then(() => {
                messageApi
                    .open({
                        type: "success",
                        content: "成功更新任务信息",
                    })
                    .then(() => {
                        window.location.reload();
                    });
            });
        } catch (error) {
            console.log(error);
        }
    };

    const handleBack = () => {
        navigate(-1);
    };
    return (
        <>
            {!loading && (
                <Fade in={true}>
                    <div>
                        {contextHolder}
                        <div>
                            <Form
                                name="basic"
                                labelCol={{span: 8}}
                                wrapperCol={{span: 16}}
                                style={{maxWidth: 600}}
                                initialValues={{
                                    title: task?.title,
                                    content: task?.content,
                                    created_at: dayjs(task?.created_at),
                                    updated_at: dayjs(task?.updated_at),
                                    user_id: task?.user_id,
                                }}
                                onFinish={onFinish}
                                autoComplete="off"
                            >
                                <Form.Item>
                                    <Button onClick={handleBack}>Back</Button>
                                </Form.Item>
                                <Form.Item label={"标题"} name={"title"}>
                                    <Input/>
                                </Form.Item>
                                <Form.Item label={"内容"} name={"content"}>
                                    <TextArea autoSize/>
                                </Form.Item>
                                <Form.Item label={"创建时间"} name={"created_at"}>
                                    <Input disabled={true}/>
                                </Form.Item>
                                <Form.Item label={"更新时间"} name={"updated_at"}>
                                    <Input disabled={true}/>
                                </Form.Item>
                                <Form.Item label={"创建用户"} name={"user_id"}>
                                    <SelectUser
                                        defaultUserId={task?.user_id}
                                        onChange={() => {
                                        }}
                                    />
                                </Form.Item>
                                <Button type="primary" htmlType="submit">
                                    Submit
                                </Button>
                            </Form>
                        </div>
                    </div>
                </Fade>
            )}
        </>
    );
}

export default TaskDetail;
