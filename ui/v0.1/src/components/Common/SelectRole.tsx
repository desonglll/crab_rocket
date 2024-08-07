import {Select} from "antd";
import {useEffect, useState} from "react";
import axios from "axios";

const SelectRole = ({
  selected_role,
  onSelectRole,
}: {
  selected_role: number | null | undefined;
  onSelectRole: (role: number) => void;
}) => {
  const [selectedRole, setSelectedRole] = useState(selected_role);
  const [role, setRole] = useState([]);
  const [loading, setLoading] = useState(true);
  const fetch_role = async () => {
    console.log("SelectRole fetch_role");
    try {
      await axios.get(`/role`).then((response) => {
        if (response.data.code == 200) {
          console.log(response.data.body);
          setRole(response.data.body.data);
        }
      });
    } catch (e) {
      console.log(e);
    }
  };

  useEffect(() => {
    console.log("SelectRole useEffect");
    fetch_role().finally(() => {
      setLoading(!loading);
    });
  }, []);

  const handleChange = (value: number) => {
    setSelectedRole(value);
    onSelectRole(value); // 将所选角色传递给父组件
  };
  return (
    <>
      {!loading && (
        <Select
          defaultValue={selectedRole}
          style={{ width: 120 }}
          onChange={handleChange}
          options={role.map((item: { role_id: number; role_name: string }) => ({
            value: item.role_id,
            label: item.role_name,
          }))}
        />
      )}
    </>
  );
};

export default SelectRole;
