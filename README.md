# rust-tokio-utilizing-full-cpu-demo

## How to push the server to 100% CPU?

https://github.com/tokio-rs/tokio/discussions/5991

```shell
cargo build --release

./target/release/echo-server

# the cpu utilization is about 60%
./target/release/echo-client 127.0.0.1:2345 20 99999999999

# the cpu utilization is about 94%
./target/release/echo-client 127.0.0.1:2345 1000 99999999999
```

## How does Tokio decide how many threads to spawn/use and when? #3858

https://github.com/tokio-rs/tokio/discussions/3858#discussioncomment-869878

https://tokio.rs/tokio/tutorial/spawning

https://docs.rs/tokio/latest/tokio/runtime/struct.Builder.html
