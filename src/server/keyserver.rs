use std::fs::create_dir_all;

use lazy_static::lazy_static;
use sled::IVec;

use sequoia_openpgp as openpgp;
use openpgp::cert::Cert;
use openpgp::serialize::Serialize;

use sled::Result;

extern crate sled;

/*

TODO
A better structure for this would probably be to index via key fingerprint.
The value would be the key with the username in the form of a comment

*/

//#[cfg(debug_assertions)]
const data_directory: &str = "/tmp/dagon/keys";

lazy_static! {
	static ref DB: sled::Db = {
		create_dir_all(data_directory).unwrap();
		sled::open(data_directory).unwrap()
	};
}

pub fn register(username: &[u8], cert: &Cert) ->anyhow::Result<()> {
	
	let pubkey = cert.clone().retain_userids(|_id| {false});

	let mut armor = Vec::new();
	pubkey.armored().serialize(&mut armor)?;

	DB.insert(armor, username).unwrap();
	Ok(())
}

/* pub fn retrieve(username: &[u8]) -> Result<Option<IVec>, sled::Error> {
	DB.get(username)
} */

//pub fn exists(username: &[u8])
pub fn exists(cert: &Cert) -> anyhow::Result<bool> {

	let pubkey = cert.clone().retain_userids(|_id| {false});

	let mut armor = Vec::new();
	pubkey.armored().serialize(&mut armor)?;

	Ok(DB.contains_key(armor)?)
}
