import { Button, Space, Table } from "antd";
import { NavLink } from "react-router-dom";
import axios from "axios";
import { useEffect, useState } from "react";
import dayjs from "dayjs";
import { Fade } from "@mui/material";
import type { Post } from "../../models/models.ts";

interface PostParams {
  user_id: number | null;
  limit: number | null;
  offset: number | null;
}

export function PostTable() {
  const [loading, setLoading] = useState(true);
  const [posts, setPosts] = useState<Post[]>([]);
  const [pagination, setPagination] = useState<Pagination>();
  const getPostsByParams = async (params: PostParams) => {
    try {
      const response = await axios.post("/post/filter", params);
      const mapped_data = response.data.body.data.map((item: Post) => {
        return {
          ...item,
          created_at: dayjs(item.created_at).format("YYYY年MM月DD日 HH:mm:ss"),
          updated_at: dayjs(item.updated_at).format("YYYY年MM月DD日 HH:mm:ss"),
        };
      });
      setPosts(mapped_data);
      setPagination(response.data.body.pagination);
    } catch (error) {
      console.log(error);
    }
  };
  useEffect(() => {
    getPostsByParams({
      user_id: null,
      limit: 10,
      offset: 0,
    }).then(() => {
      setLoading(!loading);
    });
  }, []);

  const columns = [
    {
      title: "文章标题",
      dataIndex: "title",
      key: "title",
      render: (_: string, post: Post) => (
        <NavLink to={`/post/detail/${post.post_id}`}>{post.title}</NavLink>
      ),
    },
    {
      title: "用户ID",
      dataIndex: "user_id",
      key: "user_id",
      render: (_: string, post: Post) => (
        <NavLink to={`/user/detail/${post.user_id}`}>{post.username}</NavLink>
      ),
    },
    {
      title: "状态",
      dataIndex: "status",
      key: "status",
    },
    {
      title: "创建时间",
      dataIndex: "created_at",
      key: "created_at",
    },
    {
      title: "更新时间",
      dataIndex: "updated_at",
      key: "updated_at",
    },
    {
      title: "动作",
      key: "action",
      render: (_: number, post: Post) => (
        <Space size="middle">
          <Button danger onClick={() => handleDelete(post.post_id)}>
            Delete
          </Button>
        </Space>
      ),
    },
  ];
  const handleDelete = (id: number) => {
    console.log(id);

    try {
      axios.delete(`post/${id}`).then(() => {
        window.location.reload();
      });
    } catch (e) {
      console.log(e);
    }
  };
  return (
    <>
      {
        <Fade in={!loading}>
          <Table
            size="small"
            columns={columns}
            dataSource={posts}
            rowKey={"post_id"}
            pagination={{
              showSizeChanger: true,
              showQuickJumper: true,
              onChange(page, pageSize) {
                const params: PostParams = {
                  user_id: null,
                  limit: pageSize,
                  offset: (page - 1) * pageSize,
                };
                getPostsByParams(params);
              },
              onShowSizeChange(current, size) {
                const params: PostParams = {
                  user_id: null,
                  limit: size,
                  offset: (current - 1) * size,
                };
                getPostsByParams(params);
              },
              total: pagination?.count,
            }}
          />
        </Fade>
      }
    </>
  );
}
