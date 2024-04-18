use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, Result},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let stream = listener.accept().await;

        match stream {
            Ok((mut stream, _)) => {
                println!("New connection");
                let _ = tokio::spawn(async move {
                    process_incomming_connections(&mut stream).await.unwrap();
                });
            }
            Err(e) => {
                println!("err: {}", e);
            }
        }
    }
}

async fn process_incomming_connections(stream: &mut TcpStream) -> Result<()> {
    loop {
        let mut buffer = [0; 1024];
        let f = stream.read(&mut buffer).await?;

        if f == 0 {
            break;
        }

        ping(stream).await?;
        println!("{}", String::from_utf8_lossy(&buffer));
    }

    Ok(())
}

/// Write PONG response in TcpStream
///
/// # Panics
/// Panics if can't write in TcpStream
async fn ping(stream: &mut TcpStream) -> Result<()> {
    stream.write(b"+PONG\r\n").await?;
    Ok(())
}
