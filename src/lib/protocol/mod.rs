use std::io::{Error, ErrorKind, Result};
use std::collections::HashMap;

#[derive(Debug)]
///This struct contains all data send in a single message the command and its arguments/data
pub struct MessageCommand {
	pub command: String,
	pub data: HashMap<String, Value>
}

///The DASP data in a message
pub type MessageData = HashMap<String, Value>;

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
///Either a vector of bytes or a single integer
//TODO value shoud consist of dyn [u8] or something similiar
pub enum Value {
	Integer(i32),
	Data(Vec::<u8>)
}

impl Value {
	pub fn int(&self) -> &i32 {
		if let Value::Integer(i) = self {
			i
		} else {
			panic!("Failed to deconstruct value to int")
		}
	}
	pub fn data(&self) -> &Vec::<u8> {
		if let Value::Data(d) = self {
			d
		} else {
			panic!("Failed to deconstruct value to data")
		}
	}
}

//I tried using Into<i32> but rustc hated that idea
impl From<i32> for Value {
	fn from(i: i32) -> Value {
		Value::Integer(i)
	}
}

impl From<Vec<u8>> for Value {
	fn from(v: Vec::<u8>) -> Value {
		Value::Data(v)
	}
}

impl From<&str> for Value {
	fn from(v: &str) -> Value {
		Value::Data(v.to_owned().into_bytes())
	}
}

impl From<String> for Value {
	fn from(v: String) -> Value {
		Value::Data(v.into_bytes())
	}
}

#[derive(PartialEq)]
enum DataType {
	Integer,
	Data
}

#[macro_export]
macro_rules! dasp(
    { $($key:expr => $value:expr),* } => {
        {
            let mut map = MessageData::new();
            $(
                map.insert($key.to_owned(), Value::from($value));
            )*
            map
        }
     };
);

#[macro_export]
macro_rules! error_message {
	($cmd:expr, $err:expr) => {
		encode_message(&format!("-{}",$cmd), dasp!{"msg".to_string() => $err})
	};
}

pub fn encode_message(command: &str, data: MessageData) -> Vec::<u8> {
	let mut message = Vec::<u8>::new();
	message.append(&mut format!("{}\r\n", command).into_bytes());
	message.append(&mut encode_data(data));
	message
}

pub fn encode_data(mut data: MessageData) -> Vec<u8> {
	let mut encoded = Vec::<u8>::new();
	for (key, val) in data.iter_mut() {
		match val {
			Value::Data(value) => {
				encoded.append(&mut format!("${}:{}:{}", key.len(), value.len(), key).into_bytes());
				encoded.append(value);
			},
			Value::Integer(value) => {
				encoded.append(&mut format!("#{}:{}:{}", key.len(), value.to_string().len(), key).into_bytes());
				encoded.append(&mut value.to_string().into_bytes());
			}
		}
	}
	encoded
}

//TODO
//Consider using Read here instead of iterator?
///Decode full protocol messages, the command and data
pub fn decode_message(mut stream: impl Iterator<Item=u8>) -> Result<MessageCommand> {
	
	let command = stream.by_ref().take(3).collect::<Vec::<u8>>();
	let command = String::from_utf8_lossy(&command).to_string();

	if command.len() != 3 {
		return Err(Error::new(ErrorKind::InvalidData, "Not able to parse 3 character command"));
	}
	
	if !(stream.next() == Some('\r' as u8) && stream.next() == Some('\n' as u8)) {
		return Err(Error::new(ErrorKind::InvalidData, "No 3 character command deteceted"))
	}

	let data = decode_data(stream);

	return match data {
		Ok(data) => Ok(MessageCommand {
			command: command,
			data: data
		}),
		Err(error) => {

			//Allow empty data
			if error.to_string() == "Failed to get starting byte".to_string() {
				return Ok(MessageCommand {
					command: command, 
					data: HashMap::new()
				})
			}

			return Err(error)
		}
	}
}

///Decode data in DASP format from a stream or string
pub fn decode_data(mut data: impl Iterator<Item=u8>) -> Result<MessageData>
{
	
	let mut hashmap = HashMap::new();

	loop {
		let data_type: DataType;
		match data.next() {
			Some(first_byte) => {
				match first_byte {
					b'$' => data_type = DataType::Data,
					b'#' => data_type = DataType::Integer,

					//0 Could mean that the stream closed
					//('\r') or ('\n') is regarded as the end of a message
					0 | b'\r' | b'\n' => {
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