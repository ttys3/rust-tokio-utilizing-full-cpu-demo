use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::runtime::Builder;

#[tokio::main]
async fn main() {
    // // build runtime
    // let runtime = Builder::new_multi_thread()
    //     .worker_threads(6)
    //     .thread_name("my-custom-name")
    //     .thread_stack_size(3 * 1024 * 1024)
    //     .build()
    //     .unwrap();

    let listener = TcpListener::bind("127.0.0.1:2345").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut buf = Vec::with_capacity(200);
            while let Ok(len) = socket.read_buf(&mut buf).await {
                if len == 0 {
                    break;
                }
                if let Err(_) = socket.write_all(&buf).await { // echo the request
                    break;
                }
                buf.clear();
            }
        });
    }
}