use sequoia_openpgp as openpgp;
use openpgp::cert::prelude::*;

pub fn create_key(username: &str) -> openpgp::Result<Cert> {
	let (cert, _) = CertBuilder::new()
	.add_userid(username)
	.generate()?;
	
	Ok(cert)
}
