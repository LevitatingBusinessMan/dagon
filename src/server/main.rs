use std::net::{TcpListener, TcpStream};
use std::io::{Read,Write};
use std::time::Duration;

#[macro_use]
extern crate dagon_lib;
use dagon_lib::protocol::*;

mod commands;
use commands::command_handler;

mod keyserver;
mod logger;
use logger::*;

fn main() -> std::io::Result<()> {

	linfo("info");
	lsuc("suc");
	lwarn("warn");
	lerr("err");
	ldebug("debug");

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

	//stream.write(format!("{:?}\n", data).as_bytes()).unwrap();

	let output = command_handler(data);

	print!("{}", String::from_utf8_lossy(&output));

	stream.write(&output).unwrap();
}
