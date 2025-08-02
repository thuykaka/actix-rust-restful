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

## 5. Todo

- [✔️] ~~SeaOrm: https://github.com/SeaQL/sea-query#table-create + https://www.sea-ql.org/~~
- [✔️] ~~Validation: https://github.com/ranger-ross/actix-web-validation + https://github.com/Keats/validator~~
- Using http request
- Using redis
- Queue
- Upload file
- CORS
- Ratelimit
- Swagger
- Global error handler
