- [🦀 Crab Rocket](#-crab-rocket)
  - [🧩 Project Dependencies](#-project-dependencies)
  - [⚙️ Requirements](#️-requirements)
  - [🥰 Development](#-development)
    - [Database Migration](#database-migration)
  - [🔧 Compile Release Version](#-compile-release-version)
    - [Installation](#installation)
    - [🚀 Running the Binary](#-running-the-binary)
    - [Reset Database](#reset-database)
    - [Run](#run)
  - [📖 Change Log](#-change-log)
  - [(Optional) Run with Docker](#optional-run-with-docker)
  - [(Optional) `mise`](#optional-mise)
    - [Implementation Notes](#implementation-notes)
  - [(Optional) Docker Toast](#optional-docker-toast)
  - [(Optional) Docker Compose for PostgreSQL](#optional-docker-compose-for-postgresql)
  - [(Optional) Using `posting`](#optional-using-posting)
  - [(Optional) Using `cross`](#optional-using-cross)
    - [Installation](#installation-1)
    - [Add Dependencies](#add-dependencies)
    - [Usage](#usage)

# 🦀 Crab Rocket

A web backend server written in Rust and based on Rocket.

![crab_rocket](./assets/crab,super_moden_rocket,_fast_and_complex,_in_the_universe,_full_of_stars,_delightful.png)

## 🧩 Project Dependencies

- Rust
- Rocket
- Diesel
- dotenvy
- serde_json
- chrono

## ⚙️ Requirements

- Rust 2021 environment
- PostgreSQL

## 🥰 Development

### Database Migration

```shell
# Install Diesel CLI
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/download/v2.2.1/diesel_cli-installer.sh | sh

cd ./modules/cb_schema

# Setup Diesel
diesel setup

# Redo and run migrations
diesel migration redo
diesel migration run

# Run the server
cargo run
```

!!! Everytime run `cargo test` should run `diesel database reset` first.

## 🔧 Compile Release Version

```shell
cargo build --release
```

### Installation

```shell
cargo install --path .
```

### 🚀 Running the Binary

Set the environment variable:

```shell
export DATABASE_URL=postgres://@localhost/hello_rocket
```

Alternatively, update the `.env` file in the project root.

### Reset Database

```shell
diesel database reset
```

### Run

```shell
crab_rocket
```
## 📖 Change Log

[Change Log](./CHANGELOG.md)

## (Optional) Run with Docker

```shell
docker build -t crab_rocket .
docker run --name crab_rocket_demo --rm -p 8000:8000 crab_rocket
```

## (Optional) `mise`

```shell
mise run r
```

Configured in `.mise.config.toml`

### Implementation Notes

- Ensure the sequence of fields in structs matches the database schema exactly.
- Mismatches may lead to errors in `Queryable`.


## (Optional) Docker Toast

```shell
cargo install toast
docker build -t crab_rocket .
toast
```

## (Optional) Docker Compose for PostgreSQL

```yml
version: "3.1"

services:
  db:
    image: postgres
    ports:
      - "5432:5432"
    restart: always
    environment:
      POSTGRES_PASSWORD: changemeinpred!
```

## (Optional) Using `posting`

```shell
pip install posting
```

```shell
posting --collection ./posting
```

## (Optional) Using `cross`

### Installation

```shell
cargo install cross --git https://github.com/cross-rs/cross
```

### Add Dependencies

Install toolchains for specific target platforms:

```shell
rustup target add aarch64-apple-darwin
```

### Usage

```shell
# (ONCE PER BOOT, on Linux)
# Start Docker daemon (Systemd for WSL2 and SysVinit for other systems)
sudo systemctl start docker

# Build for aarch64
cross build --target aarch64-unknown-linux-gnu

# Test for mips64
cross test --target mips64-unknown-linux-gnuabi64

# Build with LTO for powerpc
cross rustc --target powerpc-unknown-linux-gnu --release -- -C lto
```
