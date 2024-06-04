import {Button, Space, Table, message} from "antd";
import {NavLink} from "react-router-dom";
import {useEffect, useState} from "react";
import axios from "axios";
import dayjs from "dayjs";
import {Fade} from "@mui/material";
import {User} from "../../models/models.ts";

export function UserTable() {
    const [loading, setLoading] = useState(true);
    const [users, setUsers] = useState<User[]>([]);
    const [messageApi, contextHolder] = message.useMessage();

    const fetchUser = async () => {
        try {
            const response = await axios.get(`user`);
            const mapped_data = response.data.data.map((item: User) => {
                return {
                    ...item,
                    created_at: dayjs(item.created_at).format("YYYY-MM-DD HH:mm:ss"),
                    updated_at: dayjs(item.updated_at).format("YYYY-MM-DD HH:mm:ss"),
                };
            });
            setUsers(mapped_data);
        } catch (e) {
            console.log(e);
        }
    };
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
    const columns = [
        {
            title: "用户ID",
            dataIndex: "user_id",
            key: "user_id",
        },
        {
            title: "用户名",
            dataIndex: "username",
            key: "username",
            render: (_: string, user: User) => (
                <NavLink to={`/user/user-detail/${user.user_id}`}>{user.username}</NavLink>
            ),
        },
        {
            title: "角色ID",
            dataIndex: "role_id",
            key: "role_id",
        },
        {
            title: "创建时间",
            dataIndex: "created_at",
            key: "created_at",
        },
        {
            title: "邮箱",
            dataIndex: "email",
            key: "email",
        },
        // {
        //     title: '密码',
        //     dataIndex: 'password',
        //     key: 'password',
        // },
        // {
        //     title: '姓名',
        //     dataIndex: 'fullname',
        //     key: 'fullname',
        // },
        // {
        //     title: '头像链接',
        //     dataIndex: 'avatar_url',
        //     key: 'avatar_url',
        // },
        // {
        //     title: '个人简介',
        //     dataIndex: 'bio',
        //     key: 'bio',
        // },
        {
            title: "更新时间",
            dataIndex: "updated_at",
            key: "updated_at",
        },
        {
            title: "手机号码",
            dataIndex: "mobile_phone",
            key: "mobile_phone",
        },
        {
            title: "动作",
            key: "action",
            render: (_: number, user: User) => (
                <Space size="middle">
                    <Button danger onClick={() => handleDelete(user.user_id)}>
                        Delete
                    </Button>
                </Space>
            ),
        },
    ];

    useEffect(() => {
        fetchUser().then(() => {
            setLoading(!loading);
        });
    }, []);
    return (
        <>
            {!loading && (
                <Fade in={!loading}>
                    <div>
                        {contextHolder}
                        <Table
                            dataSource={users}
                            columns={columns}
                            rowKey={"user_id"}
                            size={"small"}
                        />
                    </div>
                </Fade>
            )}
        </>
    );
}
