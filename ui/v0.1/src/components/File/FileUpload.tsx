import { UploadOutlined } from "@ant-design/icons";
import type { UploadProps } from "antd";
import { Button, message, Upload } from "antd";
const host = window.location.hostname;
const props: UploadProps = {
  name: "file",
  action: `http://${host}:8000/api/upload`,
  headers: {
    authorization: "authorization-text",
  },
  onChange(info) {
    if (info.file.status !== "uploading") {
      console.log(info.file, info.fileList);
    }
    if (info.file.status === "done") {
      message.success(`${info.file.name} file uploaded successfully`);
      console.log(info);
    } else if (info.file.status === "error") {
      message.error(`${info.file.name} file upload failed.`);
    }
  },
};
function FileUpload() {
  return (
    <>
      <Upload {...props}>
        <Button icon={<UploadOutlined />}>Click to Upload</Button>
      </Upload>
    </>
  );
}

export default FileUpload;
