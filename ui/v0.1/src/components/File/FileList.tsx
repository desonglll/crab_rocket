import {Collapse, CollapseProps} from "antd";
import {FileTable} from "./FileTable";
import FileUpload from "./FileUpload";

function FileList() {
  const items: CollapseProps["items"] = [
    {
      key: "1",
      label: "This is panel header 1",
      children: <FileTable />,
    },
  ];
  const onChange = (key: string | string[]) => {
    console.log(key);
  };
  return (
    <>
      <FileUpload />
      <Collapse items={items} defaultActiveKey={["1"]} onChange={onChange} />
    </>
  );
}

export default FileList;
