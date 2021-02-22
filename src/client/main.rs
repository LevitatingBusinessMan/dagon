use std::env;

mod keys;
use keys::create_key;

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();
	if args.len() < 1 {
		panic!("Need a subcommand");
	}
	match args[0].as_str() {
		"create" => {
			if args.len() < 2 {
				panic!("No username supplied")
			}
			create_key(args[1].as_str()).unwrap();
		},
		_ => panic!("Unknown sub-command")
	}
}
