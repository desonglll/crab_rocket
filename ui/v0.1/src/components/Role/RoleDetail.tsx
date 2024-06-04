import {Fade} from "@mui/material";
import {Button, DatePicker, Form, Input, message} from "antd";
import dayjs from "dayjs";
import {useEffect, useState} from "react";
import axios from "axios";
import {useParams} from "react-router-dom";
import {BackButton} from "../Common/BackButton.tsx";
import {SelectPermission} from "../Permission/SelectPermission.tsx";

interface role {
    role_id: number,
    role_name: string,
    permissions: string,
    created_at: string,
    updated_at: string,
    description: string
}

export function RoleDetail() {
    const {role_id} = useParams();
    const [role, setRole] = useState<role>();
    const [loading, setLoading] = useState(true);
    const [messageApi, contextHolder] = message.useMessage();
    const [form] = Form.useForm(); // 使用 Form 实例

    useEffect(() => {
        const fetchPost = async () => {
            try {
                const response = await axios.get(`role/${role_id}`);
                setRole(response.data.data);
            } catch (e) {
                console.log(e);
            }
        };
        fetchPost().then(() => {
            setLoading(!loading);
        });
    }, []);
    const handleSubmit = async (data: role) => {
        try {
            console.log(data);
            // 将 created_at 转换为 UTC 时间，并格式化为您希望的日期时间格式
            data.created_at = dayjs(data.created_at).format(
                "YYYY-MM-DDTHH:mm:ss.SSSSSS"
            );

            // 将 updated_at 转换为 UTC 时间，并格式化为您希望的日期时间格式
            data.updated_at = dayjs(data.updated_at).format(
                "YYYY-MM-DDTHH:mm:ss.SSSSSS"
            );
            // Send form data to server using axios or fetch
            await axios.patch(`role/${role_id}`, data).then(() => {
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
            console.error("Error submitting form:", error);
        }
    };
    const handlePermissionChange = (event: string) => {
        console.log('role new:', event)
        form.setFieldsValue({
            permissions: event
        });
    }
    return (
        <div>
            {!loading && (
                <Fade in={true}>
                    <div>
                        {contextHolder}
                        <BackButton/>
                        <Form
                            name="basic"
                            labelCol={{span: 8}}
                            wrapperCol={{span: 16}}
                            style={{maxWidth: 600}}
                            initialValues={{
                                role_name: role?.role_name,
                                role_id: role?.role_id,
                                permissions: role?.permissions,
                                description: role?.description,
                                created_at: dayjs(role?.created_at),
                                updated_at: dayjs(role?.updated_at),
                            }}
                            onFinish={handleSubmit}
                            autoComplete="off"
                            form={form}
                        >
                            <Form.Item name={"role_name"} label={"角色名称"}>
                                <Input/>
                            </Form.Item>
                            <Form.Item name={"permissions"} label={"权限"}>
                                <SelectPermission defaultSelected={role?.permissions}
                                                  onSelectPermission={handlePermissionChange}/>
                            </Form.Item>
                            <Form.Item name={"description"} label={"描述"}>
                                <Input/>
                            </Form.Item>
                            <Form.Item name={"created_at"} label={"创建时间"}>
                                <DatePicker showTime disabled/>
                            </Form.Item>
                            <Form.Item name={"updated_at"} label={"更新时间"}>
                                <DatePicker showTime disabled/>
                            </Form.Item>
                            <Button type="primary" htmlType="submit">
                                Submit
                            </Button>
                        </Form>
                    </div>
                </Fade>
            )}
        </div>
    );
}