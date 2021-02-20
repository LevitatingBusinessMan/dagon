use std::path::Path;
use std::fs::create_dir_all;

extern crate sled;

//#[cfg(debug_assertions)]
const data_directory: &str = "/tmp/dagon/keys";
static mut database: Option<sled::Db> = None;

pub fn initialize() {
	//let path = Path::new(data_directory);
	create_dir_all(data_directory).unwrap();
	unsafe {database = Some(sled::open(data_directory).unwrap())}
}

pub fn register(username: String, authkey: String) {
	
}
