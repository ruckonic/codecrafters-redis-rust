mod models;
mod commands;
mod resp;
mod utils;

use utils::store;
use utils::config;
use utils::context::{self, Context};

use core::result::Result;
use std::io::Error;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::borrow::Cow;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::commands::Command;
use crate::resp::types::RespType;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = config::load().unwrap();
    let context = context::create_context();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.port)).await?;



    loop {
        let stream: Result<(TcpStream, SocketAddr), Error> = listener.accept().await;

        match stream {
            Ok((mut stream, _)) => {
                let mut context = context.clone();

                let _ = tokio::spawn(async move {
                    process_incoming_connections(&mut stream, &mut context)
                        .await
                        .unwrap();
                });
            }
            Err(e) => {
                println!("err: {}", e);
            }
        }
    }
}

async fn process_incoming_connections(stream: &mut TcpStream, context: &mut Arc<Mutex<Context>>) -> Result<(), Error> {
    loop {
        let mut buffer: [u8; 1024] = [0; 1024];
        let bits_len: usize = stream.read(&mut buffer).await?;

        if bits_len == 0 {
            break;
        }

        let input: Cow<'_, str> = String::from_utf8_lossy(&buffer);
        let resp_type = RespType::try_from(input.to_string()).unwrap();

        let command = Command::try_from(resp_type);

        let response: String;

        match command {
            Ok(comm) => {
                let command = comm.create_command();

                match command {
                    Ok(mut command_executable) => {
                        let mut context = context.lock().unwrap();
                        let resp = command_executable.execute(&mut context);
                        response = resp.to_string();
                    }
                    Err(err) => {
                        let resp: RespType = err.into();
                        response = resp.to_string();
                    }
                }
            }
            Err(err) => {
                let resp: RespType = err.into();
                response = resp.to_string();
            }
        }

        write(stream, response.to_string().as_bytes()).await?;
    }

    Ok(())
}

async fn write(stream: &mut TcpStream, str: &[u8]) -> Result<(), Error> {
    stream.write(str).await?;
    Ok(())
}

