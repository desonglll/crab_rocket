import {ItemType, MenuItemType} from "antd/es/menu/hooks/useItems";
import {
    FileOutlined,
    FormOutlined,
    HomeOutlined,
    SettingOutlined,
    UnorderedListOutlined,
    UploadOutlined,
} from "@ant-design/icons";

const menuItems: ItemType<MenuItemType>[] = [
  {
    label: "主页",
    key: "/greet",
    icon: <HomeOutlined />,
  },
  {
    label: "文件",
    key: "/file",
    icon: <FileOutlined />,
    children: [
      {
        label: "文件列表",
        key: "/file/list",
        icon: <UnorderedListOutlined />,
      },
      {
        label: "文件上传",
        key: "/file/upload",
        icon: <UploadOutlined />,
      },
    ],
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
      {
        label: "员工管理",
        key: "/employee",
        icon: <SettingOutlined />,
        children: [
          { label: "员工列表", key: "/employee/list" },
          { label: "员工添加", key: "/employee/new" },
        ],
      },
    ],
  },
  {
    label: "日常",
    key: "/daily",
    icon: <FormOutlined />,
    children: [
      {
        label: "推文管理",
        key: "/post",
        children: [
          { label: "推文列表", key: "/post/list" },
          { label: "推文添加", key: "/post/new" },
        ],
      },
      {
        label: "任务管理",
        key: "/task",
        children: [
          { label: "任务列表", key: "/task/list" },
          { label: "任务添加", key: "/task/new" },
        ],
      },
    ],
  },
  {
    key: "github",
    label: (
      <a
        href="https://github.com/desonglll/crab_rocket"
        target="_blank"
        rel="noopener noreferrer"
      >
        Github
      </a>
    ),
  },
];

export default menuItems;
