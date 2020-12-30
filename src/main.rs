extern crate itertools;

use itertools::Itertools;
use std::{io, io::prelude::*};

enum Operation {
	Plus,
	Minus,
	Multiply,
	Divide,
	Equal,
}

enum Bracket {
	LeftRound,
	RightRound,
	LeftSquare,
	RightSquare,
	LeftCurly,
	RightCurly,
}

enum Token {
	IntegerLiteral(f64),
	StringLiteral(String),
	Operation(Operation),
	Bracket(Bracket),
	Exit,
}

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
    	let tokens = match lexer(group_chars(input).unwrap()) {
    		Ok(value) => value,
    		Err(err) => { println!("{}", err); continue; },
    	};
    	let result = match calculate(tokens) {
    		Ok(value) => value,
    		Err(err) => { println!("{}", err); continue; },
    	};
    	println!("{}", result);
    }
}

fn group_chars(input: String) -> Option<Vec<String>> {
	//group input string by types of charaters

	let mut result = Vec::new();

	for (_, group) in input.chars().group_by(|c| c.is_digit(10) || *c == '.').into_iter() {
		for (_, group) in group.collect::<String>().chars().group_by(|c| c.is_alphabetic() || *c == '_').into_iter() {
			result.push(group.collect::<String>());
		};
	};
	Some(result)
}

fn lexer(input: Vec<String>) -> Result<Vec<Token>, String> {
	let mut result = Vec::new();

	for s in input {
		match s {
			_ if s.chars().all(|c| c.is_digit(10) || c == '.') => {
				result.push(Token::IntegerLiteral(s.parse::<f64>().expect("Error.")));
			},
			_ if s.chars().all(|c| c.is_alphabetic()) => {
				match s.as_str() {
					"quit" => result.push(Token::Exit),
					_ => result.push(Token::StringLiteral(s)),
				};
			},
			_ => {
				for c in s.chars() {
					match c {
						'+' => result.push(Token::Operation(Operation::Plus)),
						'-' => result.push(Token::Operation(Operation::Minus)),
						'*' => result.push(Token::Operation(Operation::Multiply)),
						'/' => result.push(Token::Operation(Operation::Divide)),
						'=' => result.push(Token::Operation(Operation::Equal)),
						'(' => result.push(Token::Bracket(Bracket::LeftRound)),
						')' => result.push(Token::Bracket(Bracket::RightRound)),
						'[' => result.push(Token::Bracket(Bracket::LeftSquare)),
						']' => result.push(Token::Bracket(Bracket::RightSquare)),
						'{' => result.push(Token::Bracket(Bracket::LeftCurly)),
						'}' => result.push(Token::Bracket(Bracket::RightCurly)),
						_ => return Err(format!("Unexpected token: {}", c)),
					};
				};
			},
		};
	};
	Ok(result)
}

fn calculate(input: Vec<Token>) -> Result<f64, String> {
	let mut result: f64 = 0.0;

	let mut input = input.iter().peekable();

	while let Some(token) = input.peek() {
		match token {
			Token::IntegerLiteral(i) => { 
				result = *i;
				input.next();
			},
			Token::StringLiteral(_) => {
				return Err("".to_string())
			},
			Token::Operation(Operation::Plus) => {
				input.next();
				result = result + match input.next().unwrap() {
					Token::IntegerLiteral(i) => i,
					_ => return Err("Unexpected token!".to_string())
				};
			},
			Token::Operation(Operation::Minus) => {
				input.next();
				result = result - match input.next().unwrap() {
					Token::IntegerLiteral(i) => i,
					_ => return Err("Unexpected token!".to_string())
				};
			},
			Token::Operation(Operation::Multiply) => {
				input.next();
				result = result * match input.next().unwrap() {
					Token::IntegerLiteral(i) => i,
					_ => return Err("Unexpected token!".to_string())
				};
			},
			Token::Operation(Operation::Divide) => {
				input.next();
				result = result / match input.next().unwrap() {
					Token::IntegerLiteral(i) => i,
					_ => return Err("Unexpected token!".to_string())
				};
			},
			Token::Exit => {
				use std::process;
				process::exit(0x0);
			},
			_ => return Err("".to_string())
		};
	}
	Ok(result)
}
