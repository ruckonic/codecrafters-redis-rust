mod resp;

use resp::types::RespType;
use resp::command::Command;

use core::result::Result;
use std::borrow::Cow;
use std::io::Error;
use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
     let listener = TcpListener::bind("127.0.0.1:6379").await?;

     loop {
         let stream: Result<(TcpStream, SocketAddr), Error> = listener.accept().await;

         match stream {
             Ok((mut stream, _)) => {
                 println!("New connection");
                 let _ = tokio::spawn(async move {
                     process_incoming_connections(&mut stream).await.unwrap();
                 });
             }
             Err(e) => {
                 println!("err: {}", e);
             }
         }
     }
}

async fn process_incoming_connections(stream: &mut TcpStream) -> Result<(), Error> {
    loop {
        let mut buffer: [u8; 1024] = [0; 1024];
        let bits_len: usize = stream.read(&mut buffer).await?;

        if bits_len == 0 {
            break;
        }

        let input: Cow<'_, str> = String::from_utf8_lossy(&buffer);
        let resp_type = RespType::from_str(input.to_string()).unwrap();
        let response: String;
        
        let command = Command::from_resp(resp_type);

        if command.is_err() {
            response = "-ERR unknown command\r\n".to_string();
        } else {
            response = command.unwrap().execute(); 
        } 

        write(stream, response.as_bytes()).await?;
    }

    Ok(())
}

async fn write(stream: &mut TcpStream, str: &[u8]) -> Result<(), Error> {
    stream.write(str).await?;
    Ok(())
}
