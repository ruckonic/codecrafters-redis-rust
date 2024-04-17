use std::io::{Read, Result, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let mut buffer = [0; 1024];
                _stream.read(&mut buffer).unwrap();

                let commands = String::from_utf8_lossy(&buffer);

                commands
                    .matches("ping")
                    .for_each(|_| ping(&mut _stream).unwrap());
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}

/// Write PONG response in TcpStream
///
/// # Panics
/// Panics if can't write in TcpStream
fn ping(stream: &mut TcpStream) -> Result<()> {
    stream.write(b"+PONG\r\n")?;
    Ok(())
}
