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
	EOF,
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

    	let tokens = match lexer(input) {
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

fn lexer(input: String) -> Result<Vec<Token>, String> {
	let mut result = Vec::new();

	let mut iter = input.chars().peekable();

	while let Some(c) = iter.peek() {
		match c {
			'0'..='9' => {
				let mut n = String::new();
				while let Some(c) = iter.peek() {
					if c.is_digit(10) { 
						n.push(*c); 
						iter.next(); 
					} else { break; }
				};
				result.push(Token::IntegerLiteral(n.parse::<f64>().expect("Error.")));
			},
			'a'..='z' | 'A'..='Z' | '_' => {
				let mut s = String::new();
				while let Some(c) = iter.peek() {
					if c.is_alphabetic() { 
						s.push(*c); 
						iter.next();
					} else { break; }
				};
				result.push(Token::StringLiteral(s));
			},
			_ => {
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
				iter.next();
			},
		};
	};

	Ok(result)
}

fn parser(tokens: Vec<Token>) -> () {

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
			Token::Operation(op) => match op {
				Operation::Plus => {
					input.next();
					result = result + match input.next() {
						Some(Token::IntegerLiteral(i)) => i,
						_ => return Err(format!("Unexpected token: ")),
					};
				},
				Operation::Minus => {
					input.next();
					result = result - match input.next() {
						Some(Token::IntegerLiteral(i)) => i,
						_ => return Err(format!("Unexpected token: ")),
					};					
				},
				Operation::Multiply => {
					input.next();
					result = result * match input.next() {
						Some(Token::IntegerLiteral(i)) => i,
						_ => return Err(format!("Unexpected token: ")),
					};
				},
				Operation::Divide => {
					input.next();
					result = result / match input.next() {
						Some(Token::IntegerLiteral(i)) => i,
						_ => return Err(format!("Unexpected token: ")),
					};
				},
				Operation::Equal => {
					input.next();
					
				},
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
