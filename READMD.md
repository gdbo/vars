## create .env file and set database url

.env file content:

```
DATABASE_URL="mysql://root:password@localhost/dbname"
```

## install sqlx-cli

```
cargo install sqlx-cli
```

## create database

```
sqlx database create
```

## run sql migrations

```
sqlx migrate run
```
