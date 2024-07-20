import { GetProp, Upload, UploadProps, message } from "antd";
import { useState } from "react";
type FileType = Parameters<GetProp<UploadProps, "beforeUpload">>[0];
import { LoadingOutlined, PlusOutlined } from "@ant-design/icons";
import axios from "axios";

const beforeUpload = (file: FileType) => {
  const isJpgOrPng = file.type === "image/jpeg" || file.type === "image/png";
  if (!isJpgOrPng) {
    message.error("You can only upload JPG/PNG file!");
  }
  const isLt2M = file.size / 1024 / 1024 < 2;
  if (!isLt2M) {
    message.error("Image must smaller than 2MB!");
  }
  return isJpgOrPng && isLt2M;
};

const AvatarUpload = ({
  return_uuid,
  avatar_url,
}: {
  return_uuid: (uuid: string) => void;
  avatar_url: string;
}) => {
  const [loading, setLoading] = useState(false);
  const [imageUrl, setImageUrl] = useState<string>(avatar_url);

  const customRequest = (info: any) => {
    console.log(info.file);
    try {
      const data = {
        file: info.file,
        save: true,
      };
      axios
        .post(`avatar_upload`, data, {
          headers: {
            "Content-Type": "multipart/form-data",
          },
        })
        .then((resp) => {
          console.log(resp.data);
          // Assuming your backend returns the uploaded image URL
          const uuid = resp.data.data;
          setImageUrl(`${axios.defaults.baseURL}/retrieve/${uuid}`);
          return_uuid(uuid);
          if (resp.data.code == 200) {
            setLoading(false);
          }
        });
    } catch (e) {
      console.log(e);
    }
  };

  const uploadButton = (
    <button style={{ border: 0, background: "none" }} type="button">
      {loading ? <LoadingOutlined /> : <PlusOutlined />}
      <div style={{ marginTop: 8 }}>Upload</div>
    </button>
  );
  return (
    <>
      <Upload
        name="avatar"
        listType="picture-card"
        className="avatar-uploader"
        showUploadList={false}
        action={`${axios.defaults.baseURL}/avatar_upload`}
        // beforeUpload={beforeUpload}
        customRequest={customRequest}
      >
        {imageUrl ? (
          <img src={imageUrl} alt="avatar" style={{ width: "100%" }} />
        ) : (
          uploadButton
        )}
      </Upload>
    </>
  );
};

export default AvatarUpload;
