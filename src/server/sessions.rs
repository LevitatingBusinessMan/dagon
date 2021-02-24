use std::net::SocketAddr;
use std::time::SystemTime;
use sequoia_openpgp::cert::Cert;
use std::sync::Mutex;
use std::result::Result;

use crate::logger::*;

pub struct Session {
	peer: SocketAddr,
	cert: Cert,
	time: SystemTime
}

use lazy_static::lazy_static;

lazy_static!{
	static ref sessions: Mutex<Vec<Session>> = Mutex::new(Vec::<Session>::new());
}

pub fn add_session(peer: SocketAddr, cert: Cert) -> Result<(), ()> {
	linfo(&format!("Added session {:?}", peer));
	sessions.lock().unwrap().push(Session {
		peer: peer,
		cert: cert,
		time: SystemTime::now()
	});
	Ok(())
}
