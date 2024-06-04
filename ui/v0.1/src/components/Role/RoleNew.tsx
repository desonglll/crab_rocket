import {Fade} from "@mui/material";
import {Button, DatePicker, Form, Input, message} from "antd";
import {useNavigate} from "react-router-dom";
import axios from "axios";
import dayjs from "dayjs";
import {BackButton} from "../Common/BackButton.tsx";
import {SelectPermission} from "../Permission/SelectPermission.tsx";
import {SubmittedRole} from "../../models/models.ts";


export function RoleNew() {
    const navigate = useNavigate();
    const [messageApi, contextHolder] = message.useMessage();
    const [form] = Form.useForm(); // 使用 Form 实例

    const handleSubmit = async (data: SubmittedRole) => {
        // 将 created_at 转换为 UTC 时间，并格式化为您希望的日期时间格式
        data.created_at = dayjs(data.created_at).format(
            "YYYY-MM-DDTHH:mm:ss.SSSSSS"
        );

        // 将 updated_at 转换为 UTC 时间，并格式化为您希望的日期时间格式
        data.updated_at = dayjs(data.updated_at).format(
            "YYYY-MM-DDTHH:mm:ss.SSSSSS"
        );
        try {
            console.log(`handleSubmit:`, data)
            // Send form data to server using axios or fetch
            await axios.post(`role`, data).then((response) => {
                console.log(response);
                navigate(-1);
            }).finally(() => {
                messageApi.success(`添加成功`)
            });
        } catch
            (error) {
            console.error("Error submitting form:", error);
        }
    }

    const handlePermissionChange = (event: string) => {
        console.log('role new:', event)
        form.setFieldsValue({
            permissions: event
        });
    }


    return (
        <>
            {(
                <Fade in={true}>
                    <div>
                        {contextHolder}
                        <BackButton/>
                        <Form
                            name="basic"
                            labelCol={{span: 8}}
                            wrapperCol={{span: 16}}
                            style={{maxWidth: 600}}
                            onFinish={handleSubmit}
                            initialValues={{}}
                            autoComplete="off"
                            form={form}
                        >

                            <Form.Item name={"role_name"} label={"角色名称"}>
                                <Input/>
                            </Form.Item>
                            <Form.Item name={"permissions"} label={"权限"}>
                                <SelectPermission defaultSelected={undefined}
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
        </>
    );
}