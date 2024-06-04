import {useNavigate} from "react-router-dom";
import {useEffect} from "react";
import axios from "axios";
import {Fade} from "@mui/material";
import {Button, DatePicker, Form, Input, message} from "antd";
import {BackButton} from "../Common/BackButton";
import "./UserNew.scss"
import dayjs from "dayjs";
import timezone from "dayjs/plugin/timezone"; // 引入时区插件
import utc from "dayjs/plugin/utc";
import {PatchUser} from "../../models/models.ts"; // 引入 UTC 插件
// 添加时区和 UTC 插件
dayjs.extend(timezone);
dayjs.extend(utc);

export function UserNew() {
    const [messageApi, contextHolder] = message.useMessage();
    const navigate = useNavigate();

    useEffect(() => {
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
        try {
            axios.post(`user`, data).then((resp) => {
                console.log(resp.data);
                if (resp.data.code == 200) {
                    messageApi
                        .open({
                            type: "success",
                            content: "成功添加用户信息",
                            duration: 2,
                        })
                        .then(() => {
                            navigate(-1);
                        });
                } else {
                    messageApi.open({
                        type: "warning",
                        content: resp.data.message,
                        duration: 2,
                    });
                }
            });
        } catch (error) {
            console.log(error);
        }
    };
    return (
        <>
            <Fade in={true}>
                <div className={"user-new"}>
                    {contextHolder}
                    <BackButton/>
                    <Form
                        labelCol={{span: 8}}
                        wrapperCol={{span: 16}}
                        style={{maxWidth: 600}}
                        onFinish={onFinish}
                    >
                        <Form.Item
                            name={"username"}
                            label={"Username"}
                            rules={[{required: true, message: "請輸入用戶名！"}]}
                        >
                            <Input/>
                        </Form.Item>
                        <Form.Item name={"role"} label={"role"}>
                            <Input/>
                        </Form.Item>
                        <Form.Item name={"created_at"} label={"created_at"}>
                            <DatePicker showTime/>
                        </Form.Item>
                        <Form.Item name={"email"} label={"email"}>
                            <Input/>
                        </Form.Item>
                        <Form.Item
                            name={"password"}
                            label={"password"}
                            rules={[{required: true, message: "請輸入密碼！"}]}
                        >
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
                        <Form.Item
                            name={"mobile_phone"}
                            label={"mobile_phone"}
                            rules={[{required: true, message: "請輸入手機號碼！"}]}
                        >
                            <Input/>
                        </Form.Item>
                        <Button type="primary" htmlType="submit">
                            Submit
                        </Button>
                    </Form>
                </div>
            </Fade>
        </>
    );
}
