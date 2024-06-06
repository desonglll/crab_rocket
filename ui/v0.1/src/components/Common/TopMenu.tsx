import { Menu, MenuProps } from "antd";
import { SettingOutlined } from "@ant-design/icons";
import { useEffect, useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";

const items = [
	{
		label: "主页",
		key: "greet",
		icon: <SettingOutlined />,
	},
	{
		label: "文件",
		key: "file",
		icon: <SettingOutlined />,
	},
	{
		label: "Manage",
		key: "manage",
		children: [
			{
				label: "用户管理",
				key: "user",
				icon: <SettingOutlined />,
				children: [
					{
						type: "group",
						label: "用户管理",
						children: [
							{ label: "用户列表", key: "user-list" },
							{ label: "增加用户", key: "user-new" },
						],
					},
				],
			},
			{
				label: "角色管理",
				key: "role",
				icon: <SettingOutlined />,
				children: [
					{ label: "角色列表", key: "role-list" },
					{ label: "角色添加", key: "role-new" },
				],
			},
		],
	},
	{
		label: "推文",
		key: "post",
		children: [
			{ label: "推文列表", key: "post-list" },
			{ label: "推文添加", key: "post-new" },
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

export function TopMenu() {
	const [current, setCurrent] = useState("");
	const navigate = useNavigate();
	const location = useLocation();
	useEffect(() => {
		// 根据当前的 URL 设置初始值
		const path = location.pathname.split("/").filter(Boolean).pop();
		if (path) {
			setCurrent(path);
		}
	}, [location]);

	const onClick: MenuProps["onClick"] = (e) => {
		setCurrent(e.key);
		navigate(`${e.keyPath.reverse().join("/")}`);
	};
	return (
		<>
			<Menu
				onClick={onClick}
				selectedKeys={[current]}
				mode="horizontal"
				items={items}
			/>
		</>
	);
}
