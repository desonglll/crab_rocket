import {useEffect, useState} from "react";
import axios from "axios";
import {Button, message, Space, Table} from "antd";
import dayjs from "dayjs";
import {NavLink} from "react-router-dom";

interface role {
    role_id: number,
    role_name: string,
    permissions: string,
    created_at: string,
    updated_at: string,
    description: string
}

export function RoleTable() {
    const [roles, setRoles] = useState([])
    const [loading, setLoading] = useState(true)

    const [messageApi, contextHolder] = message.useMessage();
    const fetchData = async () => {
        try {
            const data = await axios.get(`role`)
            const mapped_data = data.data.data.map((item: role) => ({
                    role_id: item.role_id,
                    role_name: item.role_name,
                    permissions: item.permissions,
                    updated_at: dayjs(item.updated_at).format("YYYY年MM月DD日 HH:mm:ss")
                }
            ))
            setRoles(mapped_data)
        } catch (e) {
            console.log(e)
        }
    }

    useEffect(() => {
        fetchData().finally(() => {
            setLoading(!loading)
        })
    }, []);
    const handleDelete = (id: number) => {
        try {
            axios.delete(`role/${id}`).then(() => {
                messageApi
                    .open({
                        type: "success",
                        content: "删除角色成功",
                    })
                    .then(() => {
                        window.location.reload();
                    });
            });
        } catch (error) {
            console.log(error);
        }
    }

    const columns = [
        {
            title: '角色ID',
            dataIndex: 'role_id',
            key: 'role_id',
            render: (_: string, role: role) => (
                <NavLink to={`/role/role-detail/${role.role_id}`}>{role.role_id}</NavLink>
            ),
        },
        {
            title: '角色名称',
            dataIndex: 'role_name',
            key: 'role_name',
            render: (_: string, role: role) => (
                <NavLink to={`/role/role-detail/${role.role_id}`}>{role.role_name}</NavLink>
            ),
        },
        {
            title: '权限',
            dataIndex: 'permissions',
            key: 'permissions',
        },
        {
            title: '更新时间',
            dataIndex: 'updated_at',
            key: 'updated_at',
        },
        {
            title: "动作",
            key: "action",
            render: (_: number, role: role) => (
                <Space size="middle">
                    <Button danger onClick={() => handleDelete(role.role_id)}>
                        Delete
                    </Button>
                </Space>
            ),
        },

    ];
    return (
        <>
            {!loading && (
                <>
                    {contextHolder}
                    <Table dataSource={roles} columns={columns} rowKey={"role_id"} size={"middle"}/>
                </>
            )}
        </>
    );
}