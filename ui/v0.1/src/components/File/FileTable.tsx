import { Button, Card, Table, TableProps, message } from "antd";
import { useEffect, useState } from "react";
import axios from "axios";
import { File } from "../../models/models.ts";
import dayjs from "dayjs";

export function FileTable() {
  const [loading, setLoading] = useState(true);
  const [files, setFiles] = useState<File[]>();
  const [messageApi, contextHolder] = message.useMessage();

  useEffect(() => {
    const fetchData = async () => {
      try {
        const resp = await axios.get(`files`);
        setFiles(resp.data.data);
      } catch (e) {
        console.log(e);
      }
    };
    fetchData().finally(() => {
      setLoading(!loading);
    });
  }, []);

  const columns: TableProps<File>["columns"] = [
    {
      title: "uuid",
      dataIndex: "id",
      key: "id",
      render: (_, file: File) => (
        <Card
          hoverable
          onClick={() => {
            const textArea = document.createElement("textarea");
            textArea.value = file.id;
            document.body.appendChild(textArea);
            textArea.select();
            try {
              const successful = document.execCommand("copy");
              const msg = successful
                ? "File ID copied to clipboard"
                : "Failed to copy file ID";
              messageApi.success(msg);
            } catch (err) {
              messageApi.error("Failed to copy file ID");
            }
            document.body.removeChild(textArea);
          }}
          style={{ width: "max-content" }}
        >
          <div>{file.id}</div>
        </Card>
      ),
    },
    {
      title: "file_name",
      dataIndex: "file_name",
      key: "file_name",
      render: (_, file: File) => (
        <Card
          hoverable
          onClick={() => {
            const textArea = document.createElement("textarea");
            textArea.value = file.id;
            document.body.appendChild(textArea);
            textArea.select();
            try {
              const successful = document.execCommand("copy");
              const msg = successful
                ? "File ID copied to clipboard"
                : "Failed to copy file ID";
              message.success(msg);
            } catch (err) {
              message.error("Failed to copy file ID");
            }
            document.body.removeChild(textArea);
          }}
          style={{ width: "max-content" }}
        >
          <div>{file.file_name}</div>
        </Card>
      ),
    },
    {
      title: "uploaded_at",
      dataIndex: "uploaded_at",
      key: "uploaded_at",
      render: (_, file: File) => (
        <Card hoverable style={{ width: "max-content" }}>
          {dayjs(file.uploaded_at).format("YYYY-MM-DDTHH:mm:ss")}
        </Card>
      ),
    },
    {
      title: "retrieve",
      render: (_, file: File) => (
        <Button href={`${axios.defaults.baseURL}/retrieve/${file.id}`}>
          retrieve
        </Button>
      ),
    },
    {
      title: "download",
      render: (_, file: File) => (
        <Button href={`${axios.defaults.baseURL}/download/${file.id}`}>
          download
        </Button>
      ),
    },
    {
      title: "bytestream",
      render: (_, file: File) => (
        <Button href={`${axios.defaults.baseURL}/byte/stream/${file.id}`}>
          bytestream
        </Button>
      ),
    },
  ];
  return (
    <>
      {!loading && (
        <>
          {contextHolder}
          <div style={{ overflow: "auto" }}>
            <Table columns={columns} dataSource={files} rowKey={"id"} />
          </div>
        </>
      )}
    </>
  );
}
