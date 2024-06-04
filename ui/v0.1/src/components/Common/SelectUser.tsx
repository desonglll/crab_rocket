import {useEffect, useState} from "react";
import axios from "axios";
import {Select} from "antd";

export function SelectUser({defaultUserId, onChange}: { defaultUserId: number | undefined, onChange: any }) {
    const [users, setUsers] = useState([]);
    const [loading, setLoading] = useState(true);
    const [selectedUserId, setSelectedUserId] = useState(defaultUserId);
    const handleUserChange = (value: number) => {
        setSelectedUserId(value);
        if (onChange) {
            onChange(value); // 将选择改变的事件传递给父组件
        }
    };
    const fetchUser = async () => {
        try {
            const response = await axios.get(`user`);
            setUsers(response.data.data);
        } catch (error) {
            console.log(error);
        }
    };
    useEffect(() => {
        fetchUser().then(() => {
            setLoading(!loading);
        });
        return () => {
        };
    }, []);
    return (
        <>
            {
                !loading && (
                    <Select
                        style={{width: 120}}
                        options={users.map(
                            (user: { user_id: number; username: string }) => ({
                                value: user.user_id,
                                label: user.username,
                            })
                        )}
                        value={selectedUserId} // 设置选中的值
                        onChange={handleUserChange} // 处理选择改变的事件
                    />
                )
            }

        </>
    );
}