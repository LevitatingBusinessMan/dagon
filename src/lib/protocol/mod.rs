use std::io::Read;
use std::io::{Error, ErrorKind, Result};
use std::collections::HashMap;

/*
Current message syntax:
"""
CMD\r\n
DASP_DATA
"""

Where CMD is always a 3 letter command
*/

/*
I am in no way whatsoever certain what the protocol will be.
I could go with capnrpoto, or messagepack, some self defined mess.
For now I cooked up this hashmap encoding (DASP).

A keyvalue pair either starts with a $ or # (string type for the first, integer for the latter)
Followed is the length of the key, a closing :
Then the length of the value, another closing :
And then the key and value

Example:
#2:1:id2$3:5:msghello
	id: 2
	msg: "hello"

Decode like:
decode(&mut "#2:1:id2$3:5:msghello".bytes())

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

#[derive(PartialEq)]
enum DataType {
	Integer,
	Data
}

///Decode full protocol messages, the command and data
pub fn decode_message(stream: &mut impl Iterator<Item=u8>) -> Result<(String, HashMap<String, Value>)> {
	
	let command = stream.take(3).collect::<Vec::<u8>>();
	let command = String::from_utf8_lossy(&command);

	if !(stream.next() == Some('\r' as u8) && stream.next() == Some('\n' as u8)) {
		return Err(Error::new(ErrorKind::InvalidData, "4th and 5th byte is not '\\r\\n'"))
	}

	let data = decode_data(stream);
	println!("{}", command);

	return match data {
		Ok(data) => Ok((command.to_string(), data)),
		Err(error) => {

			//Allow empty data
			if error.to_string() == "Failed to get starting byte".to_string() {
				return Ok((command.to_string(), HashMap::new()))
			}

			return Err(error)
		}
	}
}

///Decode data in DASP format from a stream or string
pub fn decode_data(data: &mut impl Iterator<Item=u8>) -> Result<HashMap<String, Value>>
{
	
	let mut hashmap = HashMap::new();

	loop {
		let data_type: DataType;
		match data.next() {
			Some(first_byte) => {
				match first_byte {
					b'$' => data_type = DataType::Data,
					b'#' => data_type = DataType::Integer,

					//Unwrapping the result returned an error
					0 => {
						if hashmap.len() > 0 {
							return Ok(hashmap)
						}
						return Err(Error::new(ErrorKind::InvalidData, "Failed to get starting byte"))
					}
					unknown => return Err(Error::new(ErrorKind::InvalidData, format!("Unrecognized starting byte: '{}'", unknown)))
				}
			},
			None => {
				if hashmap.len() > 0 {
					return Ok(hashmap)
				}
				return Err(Error::new(ErrorKind::InvalidData, "Expected starting byte but none found"))
			}
		}


		//#region Collecting the lengths
		let mut buf = String::new();

		loop {
			let byte = data.next().unwrap() as char;
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
			let byte = data.next().unwrap() as char;

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
			let byte = data.next().unwrap() as char;
			if !byte.is_ascii_alphanumeric() {
				return Err(Error::new(ErrorKind::InvalidData, format!("Unsupported byte in key: '{}'", byte as u8)))
			}
			key.push(byte);
		}

		let value: Value;

		if data_type == DataType::Data {
			let mut value_data = Vec::<u8>::with_capacity(valuelength as usize);
			for _ in 0..valuelength {
				let byte = data.next().unwrap();
				value_data.push(byte);
			}
			value = Value::Data(value_data);
		} else {
			let mut value_data = String::with_capacity(valuelength as usize);
			for _ in 0..valuelength {
				let byte = data.next().unwrap() as char;
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