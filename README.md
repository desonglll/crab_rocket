- [Development](#development)
- [Compile Released Version](#compile-released-version)
- [Running Executive Binary File](#running-executive-binary-file)
  - [First Running](#first-running)
  - [Second Running](#second-running)
- [Work Flow](#work-flow)

## Development

```shell
cargo run
```

## Compile Released Version

```shell
cargo build --release
```

## Running Executive Binary File

### First Running

```shell
./hello_rocket init
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
