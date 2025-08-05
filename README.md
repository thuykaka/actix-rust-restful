## 1. Run dev

```sh
$ watchexec -e rs -r cargo run
```

## 2. Run build production

```sh
$ cargo build --release
```

Docker build

```sh
docker build -t actix_rust_restful_docker .
```

## 3. Run stress test

```sh
$ docker run --rm --network host williamyeh/wrk -t12 -c400 -d30s http://127.0.0.1:3000/ping

$ docker run --rm --network host -v "%cd%\wrk-test\login.lua:/login.lua" williamyeh/wrk -t12 -c400 -d30s -s /login.lua http://192.168.1.9:3000/auth/login

$ docker run --rm --network host -v "%cd%\wrk-test\auth-me.lua:/auth-me.lua" williamyeh/wrk -t12 -c400 -d30s -s /auth-me.lua http://192.168.1.9:3000/auth/me
```

## 4. SeaORM

- `Schema`: a database with a collection of tables
- `Entity`: each table corresponds to an Entity.

  The Entity trait provides an API for you to inspect its properties (Column, Relation and PrimaryKey) at runtime.

  Each table has multiple columns, which are referred to as attribute.
  These attributes and their values are grouped in a Rust struct (a Model) so that you can manipulate them.

  However, Model is for read operations only. To perform insert, update, or delete, you need to use ActiveModel which attaches meta-data on each attribute.

- CLI uses:

```sh
# E.g. to generate `migration/src/m20220101_000001_create_user_table.rs` shown below
$ sea-orm-cli migrate generate create_user_table

# Run migration
$ sea-orm-cli migrate up

# Rollback last applied migration
$ sea-orm-cli migrate down

# Drop all tables from the database, then reapply all migrations
$ sea-orm-cli migrate fresh

# Rollback all applied migrations, then reapply all migrations
$ sea-orm-cli migrate refresh

# Rollback all applied migrations
$ sea-orm-cli migrate reset

# Create entity (discover all tables in a database and generate a corresponding SeaORM entity file for each table)

$ sea-orm-cli generate entity -o entity/src
# or $ sea-orm-cli generate entity -u protocol://username:password@localhost/bakery -o entity/src

```

## 5. Todo

- [✔️] ~~SeaOrm: https://github.com/SeaQL/sea-query#table-create + https://www.sea-ql.org/~~
- [✔️] ~~Validation: https://github.com/ranger-ross/actix-web-validation + https://github.com/Keats/validator~~
- [✔️] ~~Using http request: https://docs.rs/reqwest/latest/reqwest/~~
- Using redis
- Queue
- Upload file
- [✔️] ~~CORS: https://github.com/actix/actix-extras/tree/master/actix-cors~~
- [✔️] ~~Ratelimit: https://github.com/bigyao25/actix-web-ratelimit~~
- Swagger
- Global error handler
  - [✔️] ~~JSON payload~~
  - Form
  - Query,
  - Path
- mqtt with emqx
- websocket
- [✔️] ~~datetime db: using chrono~~
- [✔️] ~~refresh token~~

## 6. Refers

- https://www.youtube.com/watch?v=aZmrfizffL0&list=PLGOIZXklfFkRh8jHNY8070KUl86Tj3Ztf
- https://dev.to/chaudharypraveen98/form-validation-in-rust-404l
- https://github.com/trasherr/Blogging-API-Actix-Web
- https://github.com/dheshanm/ToDo-Sea-ORM
- https://github.com/SeaQL/sea-query
- https://github.com/actix/examples
- https://actix.rs/docs
