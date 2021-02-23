use std::fs::create_dir_all;

use lazy_static::lazy_static;
use sled::IVec;

extern crate sled;

//#[cfg(debug_assertions)]
const data_directory: &str = "/tmp/dagon/keys";

lazy_static! {
	static ref DB: sled::Db = {
		create_dir_all(data_directory).unwrap();
		sled::open(data_directory).unwrap()
	};
}

pub fn register(username: &[u8], key: &[u8]) -> Result<(), ()> {
	DB.insert(username, key).unwrap();
	Ok(())
}

pub fn retrieve(username: &[u8]) -> Result<Option<IVec>, sled::Error> {
	DB.get(username)
}
