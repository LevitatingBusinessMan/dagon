
use std::fs;
use std::fs::File;
use std::io::Write;

use sequoia_openpgp as openpgp;
use openpgp::cert::prelude::*;
use openpgp::serialize::Serialize;

pub fn create_key(username: &str) -> openpgp::Result<()> {

	#[allow(deprecated)]
	let path = std::env::home_dir().unwrap().join(".local/share/dagon/keys/");

	fs::create_dir_all(&path).unwrap();
	let mut keyfile = File::create(&path.join(username)).unwrap();
	
	let (cert, _) = CertBuilder::new()
	.add_userid(username)
	.generate()?;
	let key = cert.as_tsk();
	
	key.armored().export(&mut keyfile).unwrap();
	Ok(())
}

