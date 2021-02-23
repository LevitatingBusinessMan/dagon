use backtrace::{Backtrace, Symbol};
use chrono::Local;

pub fn log(ltype: &str, msg: &str) {
	let color = match ltype {
		"inf"	=> "34",
		"suc"	=> "32",
		"war"	=> "33",
		"err"	=> "31",
		"debug"	=> "5;31;43",
		_ 		=> "35"
	};

	#[allow(unused_variables)]
	let time = Local::now().format("%H:%M:%S").to_string();


	match ltype {
		"err" | "debug" => {
			let bt = Backtrace::new();
			let bt = bt.frames();
			
			let mut file = Ok(String::new());

			backtrace::resolve(bt[1].ip(), |symbol| {
				//If the 3e function is from this module, we need to go 1 deeper
				//This way we support both the helper function and direct usage of log()
				if let Some(name) = symbol.name() {
					if name.as_str().unwrap().contains("logger") {
						backtrace::resolve(bt[2].ip(), |symbol| {
							file = get_file_from_symbol(symbol);
						});
						return;
					}
				}
				file = get_file_from_symbol(symbol);
			});

			match file {
				Ok(file) => println!("[\x1b[{}m{}\x1b[0m]: {} ({})", color, ltype.to_uppercase(), msg, file),
				Err(_) => println!("[\x1b[{}m{}\x1b[0m]: {} (ukwn)", color, ltype.to_uppercase(), msg)
			}
		}
		_ => println!("[\x1b[{}m{}\x1b[0m]: {}", color, ltype.to_uppercase(), msg)
	};
}

pub fn linfo(msg: &str) {
	log("inf", msg);
}

pub fn lsuc(msg: &str) {
	log("suc", msg);
}

pub fn lwarn(msg: &str) {
	log("war", msg);
}

pub fn lerr(msg: &str) {
	log("err", msg);
}

pub fn ldebug(msg: &str) {
	log("debug", msg);
}

fn get_file_from_symbol(symbol: &Symbol) -> Result<String, ()> {
	let filename = symbol.filename().ok_or(())?.file_name().ok_or(())?.to_str().ok_or(())?;
	let lineno = symbol.lineno().ok_or(())?;
	Ok(format!("{}:{}", filename, lineno).to_owned())
}
