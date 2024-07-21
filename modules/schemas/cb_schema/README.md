
## Env

```shell
# DATABASE_URL=postgres://username:password@localhost/hello_rocket
DATABASE_URL=postgres://postgres:070011@localhost:5432/hello_rocket
```

## Run
```shell
# install diesel cli
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/download/v2.2.1/diesel_cli-installer.sh | sh
diesel setup
diesel database reset
```