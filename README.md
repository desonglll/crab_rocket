- [🦀 Crab Rocket](#-crab-rocket)
- [🧩 Project Dependencies](#-project-dependencies)
- [⚙️ Requirement](#️-requirement)
- [Development](#development)
- [Using cross](#using-cross)
  - [Install](#install)
  - [Usage](#usage)
- [🔧 Compile Released Version](#-compile-released-version)
- [🚀 Running Executive Binary File](#-running-executive-binary-file)
  - [First Running](#first-running)
  - [Second Running](#second-running)
  - [Reset Database](#reset-database)
- [Run with Dockerfile](#run-with-dockerfile)
- [🚦 Work Flow](#-work-flow)
  - [impl level](#impl-level)
- [Develop Warning](#develop-warning)
- [📖 Change Log](#-change-log)
- [Docker toast](#docker-toast)
- [Docker Compose to run Postgres](#docker-compose-to-run-postgres)

## 🦀 Crab Rocket

A web backend server written in Rust and based on Rocket.

![crab_rocket](./assets/crab,super_moden_rocket,_fast_and_complex,_in_the_universe,_full_of_stars,_delightful.png)

## 🧩 Project Dependencies

- Rust
- Rocket
- diesel
- dotenvy
- serde_json
- chrono

## ⚙️ Requirement

- Rust 2021 environment
- Postgres

## Development

```shell
diesel setup
diesel migration redo
diesel migration run
cargo run
```

## Using cross
### Install

```shell
cargo install cross --git https://github.com/cross-rs/cross
```
### Usage
```shell
# (ONCE PER BOOT, on Linux)
# Start the Docker daemon, if it's not already running using systemd
# on WSL2 and other systems using SysVinit, use `sudo service docker start`.
$ sudo systemctl start docker

# MAGIC! This Just Works
$ cross build --target aarch64-unknown-linux-gnu

# EVEN MORE MAGICAL! This also Just Works
$ cross test --target mips64-unknown-linux-gnuabi64

# Obviously, this also Just Works
$ cross rustc --target powerpc-unknown-linux-gnu --release -- -C lto
```

## 🔧 Compile Released Version

```shell
cargo build --release
```

## 🚀 Running Executive Binary File

Setting environment variable.

`export DATABASE_URL=postgres://@localhost/hello_rocket`

### First Running

```shell
diesel setup
diesel migration redo
diesel migration run
```

### Second Running

```shell
./hello_rocket
```

### Reset Database

```shell
diesel database reset
```

## Run with Dockerfile

```shell
docker build -t crab_rocket .
docker run --name crab_rocket_demo --rm -p 8000:8000 crab_rocket
```

## 🚦 Work Flow

- Change Models
- Run diesel migration
- Modify Mappers
- Modify Services
- Modify Routes
- Mount routes in Main

### impl level

Handle all error in impl level, and print out the error detail in backend.

## Develop Warning

- The sequence of the fields in struct is absolutely the same with database.
- Or Queryable will wrong.

## 📖 Change Log

[Change Log](./CHANGELOG.md)

## Docker toast

`cargo install toast`

`docker build -t crab_rocket .`

`toast`

## Docker Compose to run Postgres

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
