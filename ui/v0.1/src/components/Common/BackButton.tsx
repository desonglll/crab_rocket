import { Button } from "antd";
import { useNavigate } from "react-router-dom";

export function BackButton() {
  const navigate = useNavigate();
  const handleGoBack = () => {
    navigate(-1); // 返回上一级
  };
  return (
    <>
      <Button type="primary" onClick={handleGoBack}>
        返回
      </Button>
    </>
  );
}
