import "antd/dist/reset.css";
import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import "./index.css";
import axios from "axios";

const host = window.location.hostname;
console.log(host);
axios.defaults.baseURL = `http://${host}:8000/api`;

ReactDOM.createRoot(document.getElementById("root")!).render(<App />);
