import {useParams} from "react-router-dom";
import {useEffect, useState} from "react";
import axios from "axios";
import {Fade} from "@mui/material";
import {Button, DatePicker, Form, Input, message} from "antd";
import {BackButton} from "../Common/BackButton";
import dayjs from "dayjs";
import timezone from "dayjs/plugin/timezone"; // 引入时区插件
import utc from "dayjs/plugin/utc";
import SelectRole from "../Common/SelectRole.tsx";
import {PatchUser, User} from "../../models/models.ts";
// 添加时区和 UTC 插件
dayjs.extend(timezone);
dayjs.extend(utc);

export function UserDetail() {
    const {user_id} = useParams();
    const [loading, setLoading] = useState(true);
    const [user, setUser] = useState<User>();
    const [messageApi, contextHolder] = message.useMessage();
    const [form] = Form.useForm(); // 使用 Form 实例

    const fetchUser = async () => {
        try {
            const response = await axios.get(`user/${user_id}`);
            setUser(response.data.data);
        } catch (e) {
            console.log(e);
        }
    };
    useEffect(() => {
        fetchUser().then(() => {
            setLoading(!loading);
        });
    }, []);

    const onFinish = (data: PatchUser) => {
        // 将 created_at 转换为 UTC 时间，并格式化为您希望的日期时间格式
        data.created_at = dayjs(data.created_at).format(
            "YYYY-MM-DDTHH:mm:ss.SSSSSS"
        );

        // 将 updated_at 转换为 UTC 时间，并格式化为您希望的日期时间格式
        data.updated_at = dayjs(data.updated_at).format(
            "YYYY-MM-DDTHH:mm:ss.SSSSSS"
        );
        console.log(data)
        try {
            axios.patch(`user/${user_id}`, data).then(() => {
                messageApi
                    .open({
                        type: "success",
                        content: "成功更新用户信息",
                        duration: 2,
                    })
                    .then(() => {
                        window.location.reload();
                    });
            });
        } catch (error) {
            console.log(error);
        }
    };
    const handleSelectRole = (role: number) => {
        console.log("select:", role)
        // 使用 Form 实例的 setFieldsValue 方法更新表单字段值
        form.setFieldsValue({
            role: role,
        });
    };
    return (
        <>
            {!loading && (
                <Fade in={!loading}>
                    <div>
                        {contextHolder}
                        <BackButton/>
                        <Form
                            initialValues={{
                                username: user?.username,
                                role: user?.role_id,
                                created_at: dayjs(user?.created_at),
                                email: user?.email,
                                password: user?.password,
                                fullname: user?.fullname,
                                avatar_url: user?.avatar_url,
                                bio: user?.bio,
                                updated_at: dayjs(user?.updated_at),
                                mobile_phone: user?.mobile_phone,
                            }}
                            labelCol={{span: 8}}
                            wrapperCol={{span: 16}}
                            style={{maxWidth: 600}}
                            onFinish={onFinish}
                            form={form}
                        >
                            <Form.Item name={"username"} label={"Username"}>
                                <Input/>
                            </Form.Item>
                            <Form.Item name={"role"} label={"role"}>
                                <SelectRole selected_role={user?.role_id} onSelectRole={handleSelectRole}/>
                            </Form.Item>
                            <Form.Item name={"created_at"} label={"created_at"}>
                                <DatePicker showTime/>
                            </Form.Item>
                            <Form.Item name={"email"} label={"email"}>
                                <Input/>
                            </Form.Item>
                            <Form.Item name={"password"} label={"password"}>
                                <Input.Password placeholder="input password"/>
                            </Form.Item>
                            <Form.Item name={"fullname"} label={"fullname"}>
                                <Input/>
                            </Form.Item>
                            <Form.Item name={"avatar_url"} label={"avatar_url"}>
                                <Input/>
                            </Form.Item>
                            <Form.Item name={"bio"} label={"bio"}>
                                <Input/>
                            </Form.Item>
                            <Form.Item name={"updated_at"} label={"updated_at"}>
                                <DatePicker showTime/>
                            </Form.Item>
                            <Form.Item name={"mobile_phone"} label={"mobile_phone"}>
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
