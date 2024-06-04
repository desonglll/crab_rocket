import {Table, TableProps} from "antd";
import {useEffect, useState} from "react";
import axios from "axios";
import {File} from "../../models/models.ts";
import {NavLink} from "react-router-dom";

export function FileTable() {
    const [loading, setLoading] = useState(true);
    const [files, setFiles] = useState<File>([]);
    useEffect(() => {
        const fetchData = async () => {
            try {
                const resp = await axios.get(`files`)
                setFiles(resp.data.data)
            } catch (e) {
                console.log(e)
            }
        }
        fetchData().finally(() => {
            setLoading(!loading)
        })

    }, []);
    const columns: TableProps<File>['columns'] = [
        {
            title: 'id',
            dataIndex: 'id',
            key: 'id',
        },
        {
            title: 'file_name',
            dataIndex: 'file_name',
            key: 'file_name',
            render: (_, file: File) => <NavLink
                to={`http://localhost:8000/retrieve/${file.id}`}>{file.file_name}</NavLink>,
        },
        {
            title: 'uploaded_at',
            dataIndex: 'uploaded_at',
            key: 'uploaded_at',
        }

    ]
    return (
        <>
            {!loading && (
                <Table columns={columns} dataSource={files} rowKey={"id"}/>
            )}
        </>
    );
}