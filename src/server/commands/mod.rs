extern crate dagon_lib;
use dagon_lib::protocol::{MessageCommand, MessageData};

mod register;
use register::register;

type ArgumentList= Vec::<&'static str>;

pub fn command_handler(message: MessageCommand) -> Vec<u8> {
	return match message.command.as_str() {
		"REG" => register(message.data),
		_ => format!("-{} Unknown command\r\n", message.command).into_bytes()
	}
}

pub fn has_required(data: &MessageData, keys: ArgumentList) -> Result<(), String> {
	for key in keys {
		if !data.contains_key(key) {
			return Err(format!("Missing value '{}' required for this command", key).to_owned());
		}
	};
	Ok(())
}
