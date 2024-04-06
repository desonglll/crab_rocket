- [Project Dependencies](#project-dependencies)
- [Requirement](#requirement)
- [Development](#development)
- [Compile Released Version](#compile-released-version)
- [Running Executive Binary File](#running-executive-binary-file)
  - [First Running](#first-running)
  - [Second Running](#second-running)
- [Work Flow](#work-flow)

## Project Dependencies

- Rust
- Rocket
- diesel
- dotenvy
- serde_json
- chrono

## Requirement

- Rust environment
- Postgres

## Development

```shell
diesel migration redo
diesel migration run
cargo run
```

## Compile Released Version

```shell
cargo build --release
```

## Running Executive Binary File

Setting environment variable.
`export DATABASE_URL=postgres://@localhost/hello_rocket`

### First Running

```shell
diesel migration redo
diesel migration run
```

### Second Running

```shell
./hello_rocket
```

## Work Flow

- Change Models
- run diesel migration
- Modify Mappers
- Modify Services
- Modify Routes
- Mount routes in Main
