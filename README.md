- [🦀 Crab Rocket](#-crab-rocket)
- [🧩 Project Dependencies](#-project-dependencies)
- [⚙️ Requirement](#️-requirement)
- [Development](#development)
- [🔧 Compile Released Version](#-compile-released-version)
- [🚀 Running Executive Binary File](#-running-executive-binary-file)
  - [First Running](#first-running)
  - [Second Running](#second-running)
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
