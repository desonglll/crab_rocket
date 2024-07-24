import {Button, message, Space, Table} from "antd";
import axios from "axios";
import {useEffect, useState} from "react";
import {NavLink} from "react-router-dom";
import dayjs from "dayjs";
import {Employee} from "../../models/models.ts";

interface EmployeeParams {
  employee_id: number | null;
  limit: number;
  offset: number;
}

function EmployeeTable() {
  const [loading, setLoading] = useState(true);
  const [employees, setEmployees] = useState<Employee[]>([]);
  const [pagination, setPagination] = useState<Pagination>();
  const [messageApi, contextHolder] = message.useMessage();
  const columns = [
    {
      title: "用户ID",
      dataIndex: "employee_id",
      key: "employee_id",
    },
    {
      title: "用户名",
      dataIndex: "employee_name",
      key: "employee_name",
      render: (_: string, emp: Employee) => (
        <NavLink to={`/employee/detail/${emp.employee_id}`}>
          {emp.employee_name}
        </NavLink>
      ),
    },
    {
      title: "邮箱",
      dataIndex: "email",
      key: "email",
    },
    {
      title: "更新时间",
      dataIndex: "last_update",
      key: "last_update",
    },
    {
      title: "手机号码",
      dataIndex: "phone_number",
      key: "phone_number",
    },
    {
      title: "动作",
      key: "action",
      render: (_: number, emp: Employee) => (
        <Space size="middle">
          <Button danger onClick={() => handleDelete(emp.employee_id)}>
            Delete
          </Button>
        </Space>
      ),
    },
  ];
  const handleDelete = (id: number) => {
    try {
      axios.delete(`user/${id}`).then(() => {
        messageApi
          .open({
            type: "success",
            content: "删除用户成功",
          })
          .then(() => {
            window.location.reload();
          });
      });
    } catch (error) {
      console.log(error);
    }
  };
  const fetchEmployee = async (params: EmployeeParams) => {
    try {
      const response = await axios.post(`employee/filter`, {
        pagination: params,
      });
      const mapped_data = response.data.body.data.map((item: Employee) => {
        return {
          ...item,
          last_update: dayjs(item.last_update).format("YYYY-MM-DD HH:mm:ss"),
        };
      });
      setEmployees(mapped_data);
      setPagination(response.data.body.pagination);
      console.log(response.data);
    } catch (error) {
      console.log(error);
    }
  };
  useEffect(() => {
    const params: EmployeeParams = {
      employee_id: null,
      limit: 10,
      offset: 0,
    };
    fetchEmployee(params).then(() => {
      setLoading(!loading);
    });
  }, []);
  return (
    <>
      {!loading && (
        <div>
          {contextHolder}
          <Table
            dataSource={employees}
            columns={columns}
            rowKey={"employee_id"}
            size={"small"}
            pagination={{
              showSizeChanger: true,
              showQuickJumper: true,
              onChange(page, pageSize) {
                const params: EmployeeParams = {
                  employee_id: null,
                  limit: pageSize,
                  offset: (page - 1) * pageSize,
                };
                fetchEmployee(params);
              },
              onShowSizeChange(current, size) {
                const params: EmployeeParams = {
                  employee_id: null,
                  limit: size,
                  offset: (current - 1) * size,
                };
                fetchEmployee(params);
              },
              total: pagination?.count,
            }}
          />
        </div>
      )}
    </>
  );
}

export default EmployeeTable;
