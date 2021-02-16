use std::io::Read;
use std::io::{Error, ErrorKind, Result};
use std::collections::HashMap;

/*
I am in no way whatsoever certain what the protocol will be.
I could go with capnrpoto, or messagepack, some self defined mess.
For now I cooked up this hashmap encoding.

A keyvalue pair either starts with a $ or # (string type for the first, integer for the latter)
Followed is the length of the key, a closing :
Then the length of the value, another closing :
And then the key and value

Example:
#2:1:id2$3:5hello
	id: 2
	msg: "hello"

Regex: (\$\d:\d\w*|#\d:\d\w*\d)+

There is a lot of uknown behavior like with empty keys or values
And there is a lot of unwrapess to work out.
In general relying on the Read trait seems like a bad idea.
I should rely on an iter with defined item types.
*/

#[derive(Debug)]
pub enum Value {
	Integer(i32),
	Data(Vec::<u8>)
}

pub fn decode<T: Read>(data: T) -> Result<HashMap<String, Value>>
{
	//Turn into iterator
	let mut data = data.bytes();
	
	let mut hashmap = HashMap::new();

	loop {
		let is_integer: bool;
		match data.next() {
			Some(first_byte) => {
				match first_byte {
					Ok(b'$') => is_integer = false,
					Ok(b'#') => is_integer = true,
					Ok(unknown) => return Err(Error::new(ErrorKind::InvalidData, format!("Unrecognized starting byte: '{}'", unknown))),
					Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Failed to receive starting byte"))
				}
			},
			None => {
				if hashmap.len() > 0 {
					return Ok(hashmap)
				} else {
					return Err(Error::new(ErrorKind::InvalidData, "Expected starting byte but none found"))
				}
			}
		}


		//#region Collecting the lengths
		let mut buf = String::new();

		loop {
			let byte = data.next().unwrap().unwrap() as char;
			
			if !byte.is_digit(10) {
				if byte == ':' {
					break
				} else {
					return Err(Error::new(ErrorKind::InvalidData, format!("Unexpected character in length of key '{}'", byte as u8)))
				}
			}

			buf.push(byte);
		}

		let keylength: u32 = buf.parse().unwrap();
		let mut buf = String::new();


		loop {
			let byte = data.next().unwrap().unwrap() as char;

			if !byte.is_digit(10) {
				if byte == ':' {
					break
				} else {
					return Err(Error::new(ErrorKind::InvalidData, format!("Unexpected character in length of value '{}'", byte as u8)))
				}
			}

			buf.push(byte);
		}

		let valuelength: u32 = buf.parse().unwrap();

		// #endregion

		//Parsing the values
		let mut key = String::with_capacity(keylength as usize);
		for _ in 0..keylength {
			let byte = data.next().unwrap().unwrap() as char;
			if !byte.is_ascii_alphanumeric() {
				return Err(Error::new(ErrorKind::InvalidData, format!("Unsupported byte in key: '{}'", byte as u8)))
			}
			key.push(byte);
		}

		let value: Value;

		if !is_integer{
			let mut value_data = Vec::<u8>::with_capacity(valuelength as usize);
			for _ in 0..valuelength {
				let byte = data.next().unwrap().unwrap();
				value_data.push(byte);
			}
			value = Value::Data(value_data);
		} else {
			let mut value_data = String::with_capacity(valuelength as usize);
			for _ in 0..valuelength {
				let byte = data.next().unwrap().unwrap() as char;
				if !byte.is_digit(10) {
					return Err(Error::new(ErrorKind::InvalidData, format!("Unsupported byte in integer value: '{}'", byte as u8)))
				}
				value_data.push(byte);
			}
			value = Value::Integer(value_data.parse().unwrap());
		}

		hashmap.insert(key, value);
	
	}
}