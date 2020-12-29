use std::{io, io::prelude::*};
use std::iter::Peekable;

enum Token {
	IntegerLiteral(u32),
	Plus,
	Minus,
	Multiply,
	Divide,
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
    	println!("{}", calculate(lexer(&input)));
    }
}

fn lexer(input: &String) -> Vec<Token> {
	let mut result = Vec::new();

	let mut input_iter = input.chars().peekable();

	while let Some(&c) = input_iter.peek() {
		match c {
			'0'..='9' => {
				input_iter.next();
				let num = get_number(c, &mut input_iter);
				result.push(Token::IntegerLiteral(num));
			}
			'+' => {
				result.push(Token::Plus);
				input_iter.next();
			},
			'-' => {
				result.push(Token::Minus);
				input_iter.next();
			},
			'*' => {
				result.push(Token::Multiply);
				input_iter.next();
			},
			'/' => {
				result.push(Token::Divide);
				input_iter.next();
			}
			' ' => {
				input_iter.next();
			}
			'q' => {
				use std::process;
				process::exit(0x0);
			}
			_ => {
				println!("Unexpected character: {:?}", c);
				break;
			}
		}
	}
	result
}

fn get_number<T: Iterator<Item = char>>(c: char, iter: &mut Peekable<T>) -> u32 {
    let mut number = c.to_string().parse::<u32>().expect("The caller should have passed a digit.");
    while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<u32>()) {
        number = number * 10 + digit;
        iter.next();
    }
    number
}

fn calculate (inp: Vec<Token>) -> u32 {
	let mut result: u32 = 0;

	let mut input = inp.iter().peekable();

	while let Some(token) = input.peek() {
		match token {
			Token::IntegerLiteral(i) => { 
				result = *i;
				input.next();
			},
			Token::Plus => {
				input.next();
				result = result + match input.next().unwrap() {
					Token::IntegerLiteral(i) => i,
					_ => break,
					};
			},
			Token::Minus => {
				input.next();
				result = result - match input.next().unwrap() {
					Token::IntegerLiteral(i) => i,
					_ => break,
					};
			},
			Token::Multiply => {
				input.next();
				result = result * match input.next().unwrap() {
					Token::IntegerLiteral(i) => i,
					_ => break,
					};
			},
			Token::Divide => {
				input.next();
				result = result / match input.next().unwrap() {
					Token::IntegerLiteral(i) => i,
					_ => break,
					};
			},
		};
	}
	result
}
