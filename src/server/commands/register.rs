use dagon_lib::protocol::{MessageData, Value};
use dagon_lib::keys::verify;

use crate::commands::{ArgumentList, has_required};
use sequoia_openpgp as openpgp;
use openpgp::cert::prelude::*;
use openpgp::parse::Parse;

/*
I am not completely settled yet on how this should behave.
I am thinking about having the user send 3 values:
username
pubkey
username signed

Then we can verify that the client possesses the privkey
I also think every pubkey should correspond to a single username
*/

pub fn register(data: MessageData) -> String {

	let required_arguments: ArgumentList = vec!["username", "pubkey", "signed"];
	if let Err(error) = has_required(&data, required_arguments) {
		return error
	}

	let username = data.get("username").unwrap().data();
	let pubkey = data.get("pubkey").unwrap().data();
	let signed_username = data.get("signed").unwrap().data();

	let cert = Cert::from_bytes(pubkey.as_slice());

	//Invalid key
	if let Err(_) = cert {
		return "-REG\r\n".to_owned()
	}

	let cert = cert.unwrap();

	let msg = verify(signed_username.as_slice(), &cert).unwrap();
	println!("{:?}",String::from_utf8(msg));

	return "+REG\r\n".to_owned()
}
