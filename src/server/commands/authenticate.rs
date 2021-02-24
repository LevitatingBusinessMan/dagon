use dagon_lib::protocol::{encode_message_enc, encode_message, MessageData, Value};
use dagon_lib::keys::create_session_key;
use std::net::SocketAddr;
use crate::keyserver::exists;
use crate::commands::{ArgumentList, has_required};
use sequoia_openpgp as openpgp;
use openpgp::cert::Cert;
use openpgp::serialize::Serialize;
use openpgp::parse::Parse;
use crate::sessions::{add_session, Session};

pub fn authenticate(data: MessageData, peer: SocketAddr) -> Vec<u8> {

	let required_arguments: ArgumentList = vec!["pubkey", "sd_key"];
	if let Err(error) = has_required(&data, required_arguments) {
		return error.into_bytes();
	}

	let pubkey = data.get("pubkey").unwrap().data();
	let sd_key = data.get("sd_key").unwrap().data();

	let pubkey = Cert::from_bytes(pubkey.as_slice());

	//Invalid key
	if let Err(_) = pubkey {
		return error_message!("AUT", "Invalid pubkey")
	}

	let pubkey = pubkey.unwrap();

	if !exists(&pubkey).unwrap() {
		return error_message!("AUT", "Pubkey is not in database")
	}

	let sd_key = Cert::from_bytes(sd_key.as_slice());

	//Invalid key
	if let Err(_) = sd_key {
		return error_message!("AUT", "Invalid sd_key")
	}

	let pubkesd_keyy = sd_key.unwrap();

	add_session(peer, pubkey.clone()).unwrap();

	let cd_key = create_session_key().unwrap();
	let mut buf = Vec::new();
	cd_key.armored().export(&mut buf).unwrap();

	encode_message_enc("+AUT", dasp!{
		"cd_key" => buf
	}, None, Some(&cd_key)).unwrap()

}
