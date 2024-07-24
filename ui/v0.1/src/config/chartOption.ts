import axios from "axios";

interface ReloadCount {
  reload_date: string;
  count: number;
}
interface ReloadCountResp {
  code: number;
  message: string;
  data: ReloadCount[];
}

async function get_reload_count() {
  const resp = await axios.get<ReloadCountResp>(`reload_count`);
  return resp.data;
}

// 获取数据并配置图表
async function setupChart() {
  const reloadDates = (await get_reload_count()).data.map(
    (item) => item.reload_date
  );
  const count = (await get_reload_count()).data.map((item) => item.count);

  const option = {
    tooltip: {
      trigger: "axis",
      axisPointer: {
        type: "cross",
        crossStyle: {
          color: "#999",
        },
      },
    },
    toolbox: {
      feature: {
        dataView: { show: true, readOnly: false },
        magicType: { show: true, type: ["line", "bar"] },
        restore: { show: true },
        saveAsImage: { show: true },
      },
    },
    legend: {
      data: ["ReloadTimes"],
    },
    xAxis: [
      {
        type: "category",
        data: reloadDates,
        axisPointer: {
          type: "shadow",
        },
      },
    ],
    yAxis: [
      {
        type: "value",
        name: "times",
        // min: 0,
        // max: 1000,
        // interval: 50,
        axisLabel: {
          formatter: "{value} times",
        },
      },
      // {
      //   type: "value",
      //   name: "Temperature",
      //   min: 0,
      //   max: 25,
      //   interval: 5,
      //   axisLabel: {
      //     formatter: "{value} °C",
      //   },
      // },
    ],
    series: [
      {
        name: "ReloadTimes",
        type: "bar",
        tooltip: {
          valueFormatter: function (value: number) {
            return (value as number) + " times";
          },
        },
        data: count,
      },
      // {
      //   name: "Precipitation",
      //   type: "bar",
      //   tooltip: {
      //     valueFormatter: function (value: number) {
      //       return (value as number) + " ml";
      //     },
      //   },
      //   data: [
      //     2.6, 5.9, 9.0, 26.4, 28.7, 70.7, 175.6, 182.2, 48.7, 18.8, 6.0, 2.3,
      //   ],
      // },
      // {
      //   name: "Temperature",
      //   type: "line",
      //   yAxisIndex: 1,
      //   tooltip: {
      //     valueFormatter: function (value: number) {
      //       return (value as number) + " °C";
      //     },
      //   },
      //   data: [
      //     2.0, 2.2, 3.3, 4.5, 6.3, 10.2, 20.3, 23.4, 23.0, 16.5, 12.0, 6.2,
      //   ],
      // },
    ],
  };

  return option;
}
export default setupChart;
