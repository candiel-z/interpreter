
mod lexer;

use crate::lexer::*;

use std::{io, io::prelude::*};

fn main() {
    loop {
    	let mut input = String::new();
    	print!(">>> ");
    	io::stdout()
    		.flush()
    		.expect("Failed to flush!");
    	io::stdin()
    		.read_line(&mut input)
    		.expect("Failed to read line!");
    	input = input.trim().to_string();

    	let tokens = match lexer(input) {
    		Ok(value) => value,
    		Err(err) => { println!("{}", err); continue; },
    	};

    	println!("{:?}", tokens);
    }
}


/*
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::env;
use std::process;

fn main() {
    let input = match read_file(env::args().collect()) {
    	Ok(value) => value,
    	Err(err) => { 
    		println!("{}", err);
			process::exit(0x0);
    	},
    };

    let result = match lexer (input.trim().to_string()) {
    	Ok(value) => value,
    	Err(err) => {
    		println!("{}", err);
			process::exit(0x0);
    	},
    };

    println!("{:?}", result);
}

fn read_file(args: Vec<String>) -> Result<String, String> {
	if args.len() != 2 { return Err(format!("No path to file was given!")); }

	let path = Path::new(&args[1]);

	let mut file = match File::open(&path) {
		Ok(file) => file,
		Err(err) => return Err(format!("Could not open file {} : {}", path.display(), err)),
	};

	let mut s = String::new();
	match file.read_to_string(&mut s) {
		Err(err) => return Err(format!("Could not read file {} : {}", path.display(), err)),
		Ok(_) => (),
	}
	Ok(s)
}
*/