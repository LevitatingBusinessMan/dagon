use std::{env, io::Read};
use std::collections::HashMap;
use std::time::Duration;
use std::io::Write;

mod keys;
use keys::new_key;

use sequoia_openpgp as openpgp;
use openpgp::serialize::{Serialize, SerializeInto};
use openpgp::cert::prelude::*;
use openpgp::serialize::stream::*;
use openpgp::packet::prelude::*;
use openpgp::parse::{Parse, stream::*};
use openpgp::policy::Policy;
use openpgp::policy::StandardPolicy as P;

extern crate dagon_lib;
use dagon_lib::protocol::{encode_message, Value, MessageData, decode_message};
use dagon_lib::keys::sign_data;
use std::net::{TcpStream, SocketAddr};

const SERVER_HOST: &str = "127.0.0.1:7777";
const TIMEOUT: u64 = 5;

macro_rules! connect {
	() => {TcpStream::connect_timeout(&SERVER_HOST.parse().unwrap(), Duration::new(TIMEOUT,0)).unwrap()}
}

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();
	if args.len() < 1 {
		panic!("Need a subcommand");
	}
	match args[0].as_str() {
		"create" => {
			let username = args[1].as_str();

			if args.len() < 2 {
				panic!("No username supplied")
			}
			let cert = new_key(username).unwrap();

			println!("Saved key to keyfolder");

			let mut data = MessageData::new();
			data.insert("username".into(), Value::Data(username.into()));
			data.insert("pubkey".into(), Value::Data(cert.armored().to_vec().unwrap()));
			data.insert("signed".into(), Value::Data(sign_data(username.as_bytes(), &cert).unwrap()));

			let mut stream = connect!();
			let message = encode_message("REG".into(), data);

			stream.write_all(&message).unwrap();

			let mut buf = Vec::new();
			stream.read_to_end(&mut buf).unwrap();

			println!("{}", String::from_utf8(buf).unwrap());
		},
		"test" => {
			let mut data = MessageData::new();
			data.insert("foo".into(), Value::Data("bar".into()));
			data.insert("biz".into(), Value::Integer(2));
			let encoded = encode_message("TST".into(), data);
			println!("{:?}", String::from_utf8_lossy(&encoded));
			println!("{:?}", decode_message(encoded.clone().iter().copied()));
		},
		_ => panic!("Unknown sub-command")
	}
}
