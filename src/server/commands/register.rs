use dagon_lib::protocol::MessageData;

use crate::commands::{ArgumentList, has_required};

use crate::keyserver;

/*
I am not completely settled yet on how this should behave.
I am thinking about having the user send 3 values:
username
pubkey
username encrypted

Then we can verify that the client possesses the privkey
I also think every pubkey should correspond to a single username
*/

pub fn register(data: MessageData) -> String {

	let required_arguments: ArgumentList = vec!["username", "pubkey", "encrypted"];
	if let Err(error) = has_required(&data, required_arguments) {
		return error
	}

	let username = data.get("username");
	let pubkey = data.get("pubkey");
	let username_encrypted = data.get("encrypted");

	return "+REG\r\n".to_owned()
}