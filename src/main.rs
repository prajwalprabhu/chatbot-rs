use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn handle_client(mut stream: TcpStream)->std::io::Result<()> {
    // ...
    println!("{:?}",stream);

    // let mut buffer = String::new();
    let message = String::from("<html><h1>Hello World!</h1></html>");

    // stream.read_to_string(&mut buffer)?;
    stream.write(message.as_bytes())?;
    // let buf :[u8;1024];
    stream.flush()?;
    // println!("Buffer : {}",buffer);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }
    Ok(())
}
