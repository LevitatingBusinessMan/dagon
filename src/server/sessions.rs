use std::net::SocketAddr;
use sequoia_openpgp::cert::Cert;
use std::sync::Mutex;
use std::result::Result;

pub struct Session {
	peer: SocketAddr,
	cert: Cert
}

use lazy_static::lazy_static;

lazy_static!{
	static ref sessions: Mutex<Vec<Session>> = Mutex::new(Vec::<Session>::new());
}

pub fn add_session(peer: SocketAddr, cert: Cert) -> Result<(), ()> {
	sessions.lock().unwrap().push(Session {
		peer: peer,
		cert: cert
	});
	Ok(())
}
