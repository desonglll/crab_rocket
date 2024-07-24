import axios from "axios";
import {useEffect, useState} from "react";
import {useParams} from "react-router-dom";
import {Button, DatePicker, Form, Input, message} from "antd";
import TextArea from "antd/es/input/TextArea";
import dayjs from "dayjs";
import timezone from "dayjs/plugin/timezone"; // 引入时区插件
import utc from "dayjs/plugin/utc"; // 引入 UTC 插件
import {Fade} from "@mui/material";
import {SelectUser} from "../Common/SelectUser.tsx";
import {BackButton} from "../Common/BackButton.tsx";
import {PatchPost, Post} from "../../models/models.ts";

// 添加时区和 UTC 插件
dayjs.extend(timezone);
dayjs.extend(utc);

function PostDetail() {
  const { post_id } = useParams();
  const [post, setPost] = useState<Post>();
  const [loading, setLoading] = useState(true);
  const [messageApi, contextHolder] = message.useMessage();

  useEffect(() => {
    const fetchPost = async () => {
      try {
        const response = await axios.get(`post/${post_id}`);
        setPost(response.data.body);
      } catch (e) {
        console.log(e);
      }
    };
    fetchPost().then(() => {
      setLoading(!loading);
    });
  }, []);

  const handleSubmit = async (data: PatchPost) => {
    try {
      console.log(data);
      // 将 created_at 转换为 UTC 时间，并格式化为您希望的日期时间格式
      data.created_at = dayjs(data.created_at).format(
        "YYYY-MM-DDTHH:mm:ss.SSSSSS"
      );

      // 将 updated_at 转换为 UTC 时间，并格式化为您希望的日期时间格式
      data.updated_at = dayjs(data.updated_at).format(
        "YYYY-MM-DDTHH:mm:ss.SSSSSS"
      );
      // Send form data to server using axios or fetch
      await axios.patch(`post/${post_id}`, data).then(() => {
        messageApi
          .open({
            type: "success",
            content: "成功更新任务信息",
          })
          .then(() => {
            window.location.reload();
          });
      });
    } catch (error) {
      console.error("Error submitting form:", error);
    }
  };

  return (
    <div>
      {!loading && (
        <Fade in={true}>
          <div>
            {contextHolder}
            <Form
              name="basic"
              labelCol={{ span: 8 }}
              wrapperCol={{ span: 16 }}
              style={{ maxWidth: 600 }}
              initialValues={{
                title: post?.title,
                body: post?.body,
                user_id: post?.user_id, // 将默认选中的用户ID传递给SelectUser组件
                status: post?.status,
                created_at: dayjs(post?.created_at),
                updated_at: dayjs(post?.updated_at),
              }}
              onFinish={handleSubmit}
              autoComplete="off"
            >
              <Form.Item>
                <BackButton />
              </Form.Item>
              <Form.Item label="标题" name="title">
                <Input />
              </Form.Item>
              <Form.Item label="内容" name="body">
                <TextArea autoSize />
              </Form.Item>
              <Form.Item label="创建用户" name="user_id">
                <SelectUser defaultUserId={post?.user_id} onChange={() => {}} />
              </Form.Item>
              <Form.Item label="状态" name="status">
                <Input />
              </Form.Item>
              <Form.Item label="创建时间" name="created_at">
                <DatePicker showTime />
              </Form.Item>
              <Form.Item name={"updated_at"} label={"更新时间"}>
                <DatePicker showTime />
              </Form.Item>
              <Button type="primary" htmlType="submit">
                提交
              </Button>
            </Form>
          </div>
        </Fade>
      )}
    </div>
  );
}

export default PostDetail;
