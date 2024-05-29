# 使用官方的Rust镜像作为基础镜像
FROM rust:latest

# 设置工作目录
WORKDIR /crab_rocket

RUN apt-get update && apt-get install -y postgresql-client
# 复制项目的Cargo.toml和Cargo.lock文件到工作目录
COPY . ./
# 构建依赖，利用Docker缓存机制加快构建速度
# RUN cargo build --release
# 设置环境变量
ENV DATABASE_URL=postgres://postgres:password@host.docker.internal:15432/hello_rocket

# 编译项目
RUN cargo install --path .

# 设置容器启动时的默认命令
CMD ["crab_rocket"]
