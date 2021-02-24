use std::fs::create_dir_all;

use lazy_static::lazy_static;
use sled::IVec;

use sequoia_openpgp as openpgp;
use openpgp::cert::Cert;
use openpgp::serialize::SerializeInto;

use sled::Result;

extern crate sled;

//#[cfg(debug_assertions)]
const data_directory: &str = "/tmp/dagon/keys";

lazy_static! {
	static ref DB: sled::Db = {
		create_dir_all(data_directory).unwrap();
		sled::open(data_directory).unwrap()
	};
}

pub fn register(username: &[u8], key: &[u8]) -> Result<()> {
	DB.insert(key, username).unwrap();
	Ok(())
}

/* pub fn retrieve(username: &[u8]) -> Result<Option<IVec>, sled::Error> {
	DB.get(username)
} */

//pub fn exists(username: &[u8])
pub fn exists(pubkey: &Cert) -> Result<bool> {
	let mut buf = Vec::new();
	pubkey.armored().serialize_into(&mut buf);
	DB.contains_key(buf)
}
