import {useEffect, useState} from "react";
import type {SelectProps} from "antd";
import {Select} from "antd";
import axios from "axios";
import {Permission} from "../../models/models.ts";

export function SelectPermission({
  defaultSelected,
  onSelectPermission,
}: {
  defaultSelected: string | undefined;
  onSelectPermission: (event: string) => void;
}) {
  const [permissions, setPermissions] = useState<SelectProps["options"]>([]);
  const [loading, setLoading] = useState(true);
  useEffect(() => {
    fetchData().finally(() => {
      setLoading(!loading);
    });
  }, []);
  const fetchData = async () => {
    try {
      const response = await axios.get(`permission`);
      const mapped_data: SelectProps["options"] = response.data.body.data.map(
        (item: Permission) => ({
          label: item.permission_name,
          value: item.permission_id.toString(),
        })
      );
      setPermissions(mapped_data);
    } catch (e) {
      console.log(e);
    }
  };
  const handleChange = (value: string[]) => {
    console.log(`selected ${value}`);
    const string_value = value.join(",");
    console.log(string_value);
    onSelectPermission(string_value);
  };
  return (
    <>
      {!loading && (
        <Select
          mode="multiple"
          allowClear
          style={{ width: "100%" }}
          placeholder="Please select"
          onChange={handleChange}
          options={permissions}
          defaultValue={defaultSelected?.split(",")}
        />
      )}
    </>
  );
}
