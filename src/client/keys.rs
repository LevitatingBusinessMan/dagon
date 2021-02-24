use std::fs;
use std::fs::File;
use std::io::Read;

use sequoia_openpgp as openpgp;
use openpgp::cert::prelude::*;
use openpgp::serialize::Serialize;
use openpgp::parse::Parse;

extern crate dagon_lib;
use dagon_lib::keys::create_auth_key;

use std::os::unix::fs::PermissionsExt;


/* Bulk of this has to go the the LIB */

//TODO: permissions on file, best would be encryption
///Generates a new key and saves it
pub fn new_key(username: &str) -> openpgp::Result<Cert> {
	#[allow(deprecated)]
	let path = std::env::home_dir().unwrap().join(".local/share/dagon/keys/");

	//TODO make folder 600 perm
	fs::create_dir_all(&path).unwrap();
	let mut keyfile = File::create(&path.join(username)).unwrap();
	
	let mut perms = keyfile.metadata()?.permissions();
	perms.set_mode(0o600);
	keyfile.set_permissions(perms)?;

	let cert = create_auth_key(username, None)?;
	
	cert.as_tsk().armored().export(&mut keyfile)?;
	Ok(cert)
}

pub fn get_key(username: &str) -> openpgp::Result<Cert> {
	#[allow(deprecated)]
	let path = std::env::home_dir().unwrap().join(".local/share/dagon/keys/").join(username);

	/*
	let mut keyfile = File::open(path)?;

	let mut buf = Vec::new();
	keyfile.read_to_end(&mut buf)?;	

	Ok(Cert::from_bytes(&buf)?)
	*/

	Ok(Cert::from_file(path)?)
}
