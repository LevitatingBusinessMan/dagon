use std::net::{TcpListener, TcpStream};
use std::io::{Read,Write};
use std::time::Duration;

extern crate lib;

use lib::protocol::{decode_data, decode_message};

fn main() -> std::io::Result<()> {
	let listener = TcpListener::bind("127.0.0.1:7777")?;

    for stream in listener.incoming() {
        on_connect(stream?);
	}

    Ok(())
}

fn on_connect(mut stream: TcpStream) {

	//Maybe first 3 chars say the command
	//The response starst with + (OK) or - (ERR) followed by the same command
	stream.set_read_timeout(Some(Duration::new(0, 100000000))).unwrap(); //100ms
	let data = decode_message(&mut std::io::Read::by_ref(&mut stream).bytes().map(|x| x.unwrap_or_default())).unwrap();
	println!("{:?}", data);
	stream.write(format!("{:?}", data).as_bytes()).unwrap();
}
