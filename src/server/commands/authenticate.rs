use dagon_lib::protocol::{encode_message, MessageData, Value};
use dagon_lib::keys::create_session_key;
use std::net::SocketAddr;
use crate::keyserver::exists;
use crate::commands::{ArgumentList, has_required};
use sequoia_openpgp as openpgp;
use openpgp::{cert::Cert, serialize::SerializeInto};
use openpgp::parse::Parse;
use crate::sessions::{add_session, Session};

pub fn authenticate(data: MessageData, peer: SocketAddr) -> Vec<u8> {

	let required_arguments: ArgumentList = vec!["pubkey", "sd_key"];
	if let Err(error) = has_required(&data, required_arguments) {
		return error.into_bytes();
	}

	let pubkey = data.get("pubkey").unwrap().data();
	let sd_key = data.get("sd_key").unwrap().data();

	let cert = Cert::from_bytes(pubkey.as_slice());

	//Invalid key
	if let Err(_) = cert {
		return "-REG\r\n".to_owned().into_bytes()
	}

	let cert = cert.unwrap();

	if !exists(&cert).unwrap() {
		return error_message!("AUT", "Pubkey is not in database")
	}

	add_session()/////

	let cd_key = create_session_key().unwrap();
	let mut buf = Vec::new();
	cd_key.armored().export_into(&mut buf).unwrap();

	encode_message("+AUT", dasp!{
		"cd_key" => buf
	})

}
