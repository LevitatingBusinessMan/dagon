use std::net::{TcpListener, TcpStream};
use std::io::{Read,Write};

extern crate lib;

use lib::protocol::decode;

fn main() -> std::io::Result<()> {
	let listener = TcpListener::bind("127.0.0.1:7777")?;
	
    for stream in listener.incoming() {
        on_connect(stream?);
	}
	
	/* let stupid = vec![];
	let data = "#2:1:id2$3:5hello".bytes().for_each(|x| stupid.push(Ok(x)));
	println!("{}", decode(data)); */

    Ok(())
}

fn on_connect(mut stream: TcpStream) {
	let data = decode(&stream).unwrap();
	println!("{:?}", data);
	stream.write(format!("{:?}", data).as_bytes());
}
