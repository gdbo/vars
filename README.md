# vars - a rust blog project

## migrations

1. Install toolchain

```
$ cargo install sqlx-cli
```

2. create .env file, like:

```shell
# .env

DATABASE_URL=mysql://mysql@localhost/db_name
```

3. create/drop database

```
sqlx database create    # create database
sqlx database drop      # drop database
```

3. create and run migrations

create migrate, will create a new file in `migrations/<timestamp>-<name>.sql`:
```
sqlx migrate add <name>
```
then add your database scheme to this file

---
run migrations
```
sqlx migrate run
```

more information to read [sqlx-cli document](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)
