## 1. Run dev

$ cargo watch -x run

## 2. Run build production

$ cargo build --release

## 3. Run stress test

$ docker run --rm --network host williamyeh/wrk -t12 -c400 -d30s http://127.0.0.1:3000/ping
