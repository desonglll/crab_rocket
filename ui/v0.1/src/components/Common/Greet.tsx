import { NavLink } from "react-router-dom";

import ReactECharts from "echarts-for-react";
import setupChart from "../../config/chartOption";
import { useEffect, useState } from "react";
export function Greet() {
  const [options, setOptions] = useState<any>();

  useEffect(() => {
    async function fetchOptions() {
      const options = await setupChart();
      setOptions(options);
    }
    fetchOptions();
  }, []);
  return (
    <>
      <NavLink to="/employee/list">Go To Employee</NavLink>
      {options ? (
        <div
          style={{
            height: "100%",
            width: "100%",
            minHeight: "500px",
            minWidth: "600px",
          }}
        >
          <ReactECharts
            option={options}
            style={{ height: "100%", width: "100%" }}
          />
        </div>
      ) : (
        <p>Loading...</p>
      )}
    </>
  );
}
