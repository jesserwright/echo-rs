use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() -> std::io::Result<()> {
    // create a socket with bind on a host address and port
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // listen on that socket fd
    for stream in listener.incoming() {
        // if there is a readable stream available..
        let mut stream = stream?;

        // log the incomming address
        if let Ok(addr) = stream.peer_addr() {
            println!("Incoming Address: {}", addr);
        }

        // allocate a buffer and start reading the stream into it
        let mut receive_buffer = String::new();
        stream.read_to_string(&mut receive_buffer)?;
        println!("Incoming Message: {}", &receive_buffer);

        // write back the message on the stream
        stream.write_all(receive_buffer.as_bytes())?;

        // close the stream - close() is ran when resource is dropped
    }

    Ok(())
}
