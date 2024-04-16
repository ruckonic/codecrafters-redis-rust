use std::io::{Result, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                ping(&_stream).unwrap();
                _stream.flush().unwrap();
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
fn ping(mut stream: &TcpStream) -> Result<()> {
    stream.write(b"+PONG\r\n")?;

    Ok(())
}
