use std::fs;
use std::fs::File;
use std::io::Write;

use sequoia_openpgp as openpgp;
use openpgp::cert::prelude::*;
use openpgp::serialize::Serialize;

extern crate dagon_lib;
use dagon_lib::keys::create_key;

/* Bulk of this has to go the the LIB */

//TODO: permissions on file, best would be encryption
///Generates a new key and saves it
pub fn new_key(username: &str) -> openpgp::Result<Cert> {
	#[allow(deprecated)]
	let path = std::env::home_dir().unwrap().join(".local/share/dagon/keys/");

	fs::create_dir_all(&path).unwrap();
	let mut keyfile = File::create(&path.join(username)).unwrap();
	
	let cert = create_key(username, None)?;
	
	cert.as_tsk().armored().export(&mut keyfile)?;
	Ok(cert)
}
