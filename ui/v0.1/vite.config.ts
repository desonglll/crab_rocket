import {defineConfig} from "vite";
import react from "@vitejs/plugin-react";
import {visualizer} from "rollup-plugin-visualizer";
import viteCompression from "vite-plugin-compression";
import viteImagemin from "vite-plugin-imagemin";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react(),
    visualizer({
      // 打包完成后自动打开浏览器，显示产物体积报告
      open: true,
    }),
  ],
  server: {
    host: "0.0.0.0",
  },
  build: {
    //移除生产环境log
    minify: "terser",
    terserOptions: {
      compress: {
        //生产环境时移除console
        drop_console: true,
        drop_debugger: true,
      },
    },
    rollupOptions: {
      output: {
        chunkFileNames: "js/[name]-[hash].js", // 引入文件名的名称
        entryFileNames: "js/[name]-[hash].js", // 包的入口文件名称
        assetFileNames: "[ext]/[name]-[hash].[ext]", // 资源文件像 字体，图片等
      },
      plugins: [
        viteCompression({
          algorithm: "gzip",
          threshold: 10240,
          verbose: true, // 是否在控制台中输出压缩结果
          ext: ".gz",
          deleteOriginFile: false, // 源文件压缩后是否删除
        }),
        viteImagemin({
          gifsicle: {
            // gif图片压缩
            optimizationLevel: 3, // 选择1到3之间的优化级别
            interlaced: false, // 隔行扫描gif进行渐进式渲染
            // colors: 2 // 将每个输出GIF中不同颜色的数量减少到num或更少。数字必须介于2和256之间。
          },
          optipng: {
            // png
            optimizationLevel: 7, // 选择0到7之间的优化级别
          },
          mozjpeg: {
            // jpeg
            quality: 20, // 压缩质量，范围从0(最差)到100(最佳)。
          },
          pngquant: {
            // png
            quality: [0.8, 0.9], // Min和max是介于0(最差)到1(最佳)之间的数字，类似于JPEG。达到或超过最高质量所需的最少量的颜色。如果转换导致质量低于最低质量，图像将不会被保存。
            speed: 4, // 压缩速度，1(强力)到11(最快)
          },
          svgo: {
            // svg压缩
            plugins: [
              {
                name: "removeViewBox",
              },
              {
                name: "removeEmptyAttrs",
                active: false,
              },
            ],
          },
        }),
      ],
    },
  },
});
