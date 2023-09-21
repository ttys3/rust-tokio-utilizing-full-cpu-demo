use std::env;
use std::time::Instant;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt, self};
// use tokio::runtime::Builder;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: {} server-address #task #request", args[0]);
        return;
    }
    let ntask = args[2].parse::<usize>().unwrap();
    let nreq = args[3].parse::<usize>().unwrap();

    // let runtime = Builder::new_multi_thread()
    //     .worker_threads(6)
    //     .thread_name("my-custom-name")
    //     .thread_stack_size(3 * 1024 * 1024)
    //     .build()
    //     .unwrap();

    let mut handlers = Vec::new();
    for _ in 0..ntask {
        let mut conn = TcpStream::connect(&args[1]).await.unwrap();
        handlers.push(tokio::spawn(async move {
            let start = Instant::now();
            for _ in 0..nreq {
                request(&mut conn).await.unwrap();
            }
            println!("task time: {:?}", start.elapsed());
        }));
    }

    for h in handlers {
        let _ = h.await;
    }
}

async fn request(stream: &mut TcpStream) -> io::Result<()> {
    stream.write_all("hello, tokio!".as_bytes()).await?;

    let mut buf = [0; 100];
    stream.read(&mut buf).await?;
    Ok(())
}