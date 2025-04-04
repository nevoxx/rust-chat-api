# Readme

## Prerequisites

You need to install have rust and cargo installed. 

If you want to use DB migrations using `sqlx` u need to install `sqlx-cli` as well:

```shell
cargo install sqlx-cli
```

## 1. Create a migration

To create a new migration you can run the command:

```shell
sqlx migrate add -r my_migration_name
```

This will generate a up and down migration file at `migrations/timestamp_my_migration_name.sql` directory.

## 2. Run migrations

Run migrations:

```shell
sqlx migrate run
```

You can revert the migration by running:

```shell
sqlx migrate revert
```

You will find a `_sqlx_migrations` table, which keeps track of the applied migrations.