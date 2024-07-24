import {Menu} from "antd";
import React from "react";

function TopMenu() {
  const items = [];

  for (let i = 0; i < 10; i += 1) {
    items.push({
      label: `Item ${i}`,
      key: i,
    });
  }

  const sharedStyle = { flex: "0 0 50px", height: 20, background: "red" };

  return (
    <div>
      <Menu
        style={{ flex: "auto", minWidth: 0 }}
        mode="horizontal"
        items={items}
      />
    </div>
  );
}

export default TopMenu;
