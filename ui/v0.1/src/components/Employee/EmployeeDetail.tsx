import { useParams } from "react-router-dom";
import { useEffect, useState } from "react";
import axios from "axios";
import { Fade } from "@mui/material";
import { Button, DatePicker, Form, Input, message } from "antd";
import { BackButton } from "../Common/BackButton";
import dayjs from "dayjs";
import timezone from "dayjs/plugin/timezone"; // 引入时区插件
import utc from "dayjs/plugin/utc";
import SelectRole from "../Common/SelectRole.tsx";
import { Employee } from "../../models/models.ts";
// 添加时区和 UTC 插件
dayjs.extend(timezone);
dayjs.extend(utc);

export function EmployeeDetail() {
  const { employee_id } = useParams();
  const [loading, setLoading] = useState(true);
  const [employee, setEmployee] = useState<Employee>();
  const [messageApi, contextHolder] = message.useMessage();
  const [form] = Form.useForm(); // 使用 Form 实例

  const fetchUser = async () => {
    try {
      const response = await axios.get(`employee/${employee_id}`);
      console.log(response.data);
      setEmployee(response.data.body);
    } catch (e) {
      console.log(e);
    }
  };
  useEffect(() => {
    fetchUser().then(() => {
      setLoading(!loading);
    });
  }, []);

  const onFinish = (data: Employee) => {
    // 将 created_at 转换为 UTC 时间，并格式化为您希望的日期时间格式
    data.last_update = dayjs(data.last_update).format(
      "YYYY-MM-DDTHH:mm:ss.SSSSSS"
    );

    data.date_of_birth = dayjs(data.date_of_birth).format(
      "YYYY-MM-DDTHH:mm:ss.SSSSSS"
    );
    data.hire_date = dayjs(data.hire_date).format("YYYY-MM-DDTHH:mm:ss.SSSSSS");
    if (data.department_id !== null) {
      data.department_id = Number(data.department_id);
    }
    console.log(data);
    try {
      axios.patch(`employee/${employee_id}`, data).then(() => {
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
    console.log("select:", role);
    // 使用 Form 实例的 setFieldsValue 方法更新表单字段值
    form.setFieldsValue({
      role_id: role,
    });
  };
  return (
    <>
      {!loading && (
        <Fade in={!loading}>
          <div>
            {contextHolder}
            <BackButton />
            <Form
              initialValues={{
                employee_id: employee?.employee_id,
                first_name: employee?.first_name,
                last_name: employee?.last_name,
                employee_name: employee?.employee_name,
                gender: employee?.gender,
                date_of_birth: dayjs(employee?.date_of_birth),
                hire_date: dayjs(employee?.hire_date),
                email: employee?.email,
                phone_number: employee?.phone_number,
                department_id: employee?.department_id,
                job_title: employee?.job_title,
                salary: employee?.salary,
                manager_id: employee?.manager_id,
                address: employee?.address,
                city: employee?.city,
                state: employee?.state,
                postal_code: employee?.postal_code,
                valid: employee?.valid,
                last_update: dayjs(employee?.last_update),
                role_name: employee?.role_name,
                role_id: employee?.role_id,
              }}
              labelCol={{ span: 8 }}
              wrapperCol={{ span: 16 }}
              style={{ maxWidth: 600 }}
              onFinish={onFinish}
              form={form}
            >
              <Form.Item name={"first_name"} label={"first_name"}>
                <Input />
              </Form.Item>
              <Form.Item name={"last_name"} label={"last_name"}>
                <Input />
              </Form.Item>
              <Form.Item name={"employee_name"} label={"employee_name"}>
                <Input />
              </Form.Item>
              <Form.Item name={"gender"} label={"gender"}>
                <Input />
              </Form.Item>
              <Form.Item name={"date_of_birth"} label={"date_of_birth"}>
                <DatePicker showTime />
              </Form.Item>
              <Form.Item name={"hire_date"} label={"hire_date"}>
                <DatePicker showTime />
              </Form.Item>
              <Form.Item name={"email"} label={"email"}>
                <Input />
              </Form.Item>
              <Form.Item name={"phone_number"} label={"phone_number"}>
                <Input />
              </Form.Item>
              <Form.Item name={"department_id"} label={"department_id"}>
                <Input />
              </Form.Item>
              <Form.Item name={"job_title"} label={"job_title"}>
                <Input />
              </Form.Item>
              <Form.Item name={"salary"} label={"salary"}>
                <Input />
              </Form.Item>
              <Form.Item name={"manager_id"} label={"manager_id"}>
                <Input />
              </Form.Item>
              <Form.Item name={"address"} label={"address"}>
                <Input />
              </Form.Item>
              <Form.Item name={"city"} label={"city"}>
                <Input />
              </Form.Item>
              <Form.Item name={"state"} label={"state"}>
                <Input />
              </Form.Item>
              <Form.Item name={"postal_code"} label={"postal_code"}>
                <Input />
              </Form.Item>
              <Form.Item name={"valid"} label={"valid"}>
                <Input />
              </Form.Item>
              <Form.Item name={"last_update"} label={"last_update"}>
                <DatePicker showTime />
              </Form.Item>
              <Form.Item name={"role_name"} label={"role_name"}>
                <Input />
              </Form.Item>
              <Form.Item name={"role_id"} label={"role_id"}>
                <SelectRole
                  selected_role={employee?.role_id}
                  onSelectRole={handleSelectRole}
                />
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
