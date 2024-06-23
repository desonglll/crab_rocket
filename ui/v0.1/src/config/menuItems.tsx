import { ItemType, MenuItemType } from "antd/es/menu/hooks/useItems";
import {
  FormOutlined,
  SettingOutlined,
  HomeOutlined,
  FileOutlined,
} from "@ant-design/icons";
const menuItems: ItemType<MenuItemType>[] = [
  {
    label: "主页",
    key: "/greet",
    icon: <HomeOutlined />,
  },
  {
    label: "文件",
    key: "/file/list",
    icon: <FileOutlined />,
  },
  {
    label: "管理",
    key: "/manage",
    icon: <SettingOutlined />,
    children: [
      {
        label: "用户管理",
        key: "/user",
        icon: <SettingOutlined />,
        children: [
          {
            type: "group",
            label: "用户管理",
            children: [
              { label: "用户列表", key: "/user/list" },
              { label: "增加用户", key: "/user/new" },
            ],
          },
        ],
      },
      {
        label: "角色管理",
        key: "/role",
        icon: <SettingOutlined />,
        children: [
          { label: "角色列表", key: "/role/list" },
          { label: "角色添加", key: "/role/new" },
        ],
      },
    ],
  },
  {
    label: "推文",
    key: "/post",
    icon: <FormOutlined />,
    children: [
      { label: "推文列表", key: "/post/list" },
      { label: "推文添加", key: "/post/new" },
    ],
  },
  {
    key: "alipay",
    label: (
      <a href="https://ant.design" target="_blank" rel="noopener noreferrer">
        Navigation Four - Link
      </a>
    ),
  },
];

export default menuItems;
