use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let socket = TcpSocket::new_v4()?;
    socket.set_reuseaddr(true)?;
    socket.bind(addr)?;
    let listener = socket.listen(1024)?;

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