# cb_utils

cb_utils 是一个用 Rust 编写的实用工具库，提供了创建目录和获取北京时间的功能。该项目还包括单元测试，以确保功能的正确性。

## 项目结构

```
cb_utils/
├── Cargo.lock
├── Cargo.toml
├── README.md
├── src
│   ├── lib.rs
│   ├── mkdir.rs
│   └── time.rs
└── upload
```

## 功能

- **mkdir**: 提供创建目录的功能，并输出成功或失败的信息。
- **time**: 提供获取当前北京时间的功能。

## 安装

```shell
cargo add crab_rocket_utils
```

## 使用示例

### 创建目录

使用 `make_directory` 函数创建一个目录，并打印结果:

```rust
use cb_utils::mkdir::make_directory;

fn main() {
    let path = "upload";
    make_directory(path);
}
```

### 获取北京时间

使用 `get_e8_time` 函数获取当前的北京时间:

```rust
use cb_utils::time::get_e8_time;

fn main() {
    let beijing_time = get_e8_time();
    println!("Current Beijing Time: {}", beijing_time);
}
```
