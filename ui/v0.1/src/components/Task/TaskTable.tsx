import {Button, Space, Table} from "antd";
import {NavLink} from "react-router-dom";
import axios from "axios";
import {useEffect, useState} from "react";
import dayjs from "dayjs";
import {Fade} from "@mui/material";
import {Task} from "../../models/models.ts";

interface TaskParams {
    user_id: number | null;
    limit: number | null;
    offset: number | null;
}

export function TaskTable() {
    const [loading, setLoading] = useState(true);
    const [tasks, setTasks] = useState<Task[]>([]);
    const [info, setInfo] = useState<Info>();
    const handleDelete = (id: number) => {
        console.log(id);
        try {
            axios.delete(`task/${id}`).then(() => {
                window.location.reload();
            });
        } catch (e) {
            console.log(e);
        }
    };
    const getTasksByParams = async (params: TaskParams) => {
        try {
            const response = await axios.post("/task/filter", params);
            console.log(response.data);
            const mapped_data = response.data.data.map((item: Task) => {
                return {
                    ...item,
                    created_at: dayjs(item.created_at).format("YYYY-MM-DD HH:mm:ss"),
                    updated_at: dayjs(item.updated_at).format("YYYY-MM-DD HH:mm:ss"),
                };
            });
            setTasks(mapped_data);
        } catch (error) {
            console.log(error);
        }
    };
    const fetchInfo = async () => {
        try {
            const response = await axios.get(`info`);
            console.log(response.data);
            setInfo(response.data.data);
        } catch (error) {
            console.log(error);
        }
    };
    useEffect(() => {
        getTasksByParams({
            user_id: null,
            limit: 10,
            offset: 0,
        }).then(() => {
            fetchInfo().then(() => {
                setLoading(!loading);
            });
        });
    }, []);
    const columns = [
        {
            title: "Title",
            dataIndex: "title",
            key: "title",
            render: (_: any, task: Task) => (
                <NavLink to={`/task/${task.id}`}>{task.title}</NavLink>
            ),
        },
        {
            title: "User ID",
            dataIndex: "user_id",
            key: "user_id",
        },
        {
            title: "created_at",
            dataIndex: "created_at",
            key: "created_at",
        },
        {
            title: "updated_at",
            dataIndex: "updated_at",
            key: "updated_at",
        },
        {
            title: "Action",
            key: "action",
            render: (_: number, task: Task) => (
                <Space size="middle">
                    <Button danger onClick={() => handleDelete(task.id)}>
                        Delete
                    </Button>
                </Space>
            ),
        },
    ];
    return (
        <>
            {!loading && (
                <Fade in={!loading}>
                    <Table
                        size="small"
                        columns={columns}
                        dataSource={tasks}
                        rowKey={"id"}
                        pagination={{
                            showSizeChanger: true,
                            showQuickJumper: true,
                            total: info?.task_count,
                            onChange(page, pageSize) {
                                const params: TaskParams = {
                                    user_id: null,
                                    limit: pageSize,
                                    offset: (page - 1) * pageSize,
                                };
                                getTasksByParams(params);
                            },
                            onShowSizeChange(current, size) {
                                const params: TaskParams = {
                                    user_id: null,
                                    limit: size,
                                    offset: (current - 1) * size,
                                };
                                getTasksByParams(params);
                            },
                        }}
                    />
                </Fade>
            )}
        </>
    );
}
