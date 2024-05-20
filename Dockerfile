# 使用 Ubuntu 基础镜像
FROM ubuntu:latest


# 更新软件包列表并安装基本工具
RUN apt-get update && apt-get install -y curl build-essential libpq-dev

# 安装 Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# 设置环境变量
ENV PATH="/root/.cargo/bin:${PATH}"

# 创建工作目录
WORKDIR /app

# 复制当前目录内容到镜像的工作目录中
COPY . .

# 编译和运行项目
RUN . $HOME/.cargo/env && cargo build && cargo run
