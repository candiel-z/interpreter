#[derive(Debug)]
pub enum Token {
	Bracket(Bracket),
	Literal(Literal),
	Keyword(Keyword),
	Operator(Operator),
	Separator(Separator),
}

#[derive(Debug)]
pub enum Bracket {
	LeftRound,
	RightRound,
	LeftSquare,
	RightSquare,
	LeftCurly,
	RightCurly,
}

#[derive(Debug)]
pub enum Literal {
	Identifier(String),
	Integer(i64),
	Float(f64),
	Bool(bool),
	Char(char),
	String(String),
}

#[derive(Debug)]
pub enum Keyword {
	If,
	Else,
	Elif,
	Loop,
	Break,
	Function,
	Return,
	Import,
	As,
}

#[derive(Debug)]
pub enum Operator {
	Plus,
	Minus,
	Multiply,
	Divide,

	Assign,

	Greater,
	GreaterOrEqual,
	Less,
	LessOrEqual,
	Equal,
	NotEqual,

	Not,
}

#[derive(Debug)]
pub enum Separator {
	Dot,
	Comma,
	Colon,
	Semicolon,
}

pub fn lexer(input: String) -> Result<Vec<Token>, String> {
	let mut result = Vec::new();
	let mut iter = input.chars().peekable();

 	while let Some(c) = iter.next() {
 		match c {
 			' ' => { continue; },
 			'(' => { 
	 			result.push(Token::Bracket(Bracket::LeftRound));
	 		},
	 		')' => { 
	 			result.push(Token::Bracket(Bracket::RightRound));
	 		},
	 		'[' => { 
	 			result.push(Token::Bracket(Bracket::LeftSquare));
	 		},
	 		']' => { 
	 			result.push(Token::Bracket(Bracket::RightSquare));
	 		},
	 		'{' => { 
	 			result.push(Token::Bracket(Bracket::LeftCurly));
	 		},
	 		'}' => { 
	 			result.push(Token::Bracket(Bracket::RightCurly));
	 		},
	 		'.' => {
	 			result.push(Token::Separator(Separator::Dot));
	 		},
	 		',' => {
	 			result.push(Token::Separator(Separator::Comma));
	 		},
	 		':' => {
	 			result.push(Token::Separator(Separator::Colon));
	 		},
	 		';' => {
	 			result.push(Token::Separator(Separator::Semicolon));
	 		},
	 		'\'' => {
	 			let c = match iter.next() {
	 				Some(c) => c,
	 				None => return Err(format!("Unexpected character: {}", ' ')),
	 			};
	 			if iter.peek() == Some(&'\'') {
	 				result.push(Token::Literal(Literal::Char(c)));
	 			} else {
	 				match iter.peek() {
	 					Some(c) => return Err(format!("Unexpected character: {}", c)),
	 					None => return Err(format!("Unexpected character: {}", ' ')),
	 				};
	 			};
	 			iter.next();
	 		},
	 		'"' => {
	 			let mut s: Vec<char> = Vec::new();
	 			while let Some(c) = iter.next() {
	 				match c {
	 					'"' => result.push(Token::Literal(Literal::String(s.iter().collect()))),
	 					_ => s.push(c),
	 				};
	 			};
	 		},
	 		'+' => {
	 			result.push(Token::Operator(Operator::Plus));
	 		},
	 		'-' => {
	 			result.push(Token::Operator(Operator::Minus));
	 		},
	 		'*' => {
	 			result.push(Token::Operator(Operator::Multiply));
	 		},
	 		'/' => {
	 			result.push(Token::Operator(Operator::Divide));
	 		},
	 		'=' => {
	 			match iter.peek() {
	 				Some('=') => {
	 					result.push(Token::Operator(Operator::Equal));
	 					iter.next();
	 				},
	 				_ => result.push(Token::Operator(Operator::Assign)),
	 			};
	 		},
	 		'>' => {
	 			match iter.peek() {
	 				Some('=') => {
	 					result.push(Token::Operator(Operator::GreaterOrEqual));
	 					iter.next();
	 				},
	 				_ => result.push(Token::Operator(Operator::Greater)),
	 			};
	 		},
	 		'<' => {
	 			match iter.peek() {
	 				Some('=') => {
	 					result.push(Token::Operator(Operator::LessOrEqual));
	 					iter.next();
	 				},
	 				_ => result.push(Token::Operator(Operator::Less)),
	 			};
	 		},
	 		'!' => {
	 			match iter.peek() {
	 				Some('=') => {
	 					result.push(Token::Operator(Operator::NotEqual));
	 					iter.next();
	 				},
	 				_ => result.push(Token::Operator(Operator::Not)),
	 			};
	 		},
	 		'0'..='9' => {
	 			let mut n = Vec::new();
	 			let mut is_float = false;

	 			n.push(c);

	 			while let Some(c) = iter.peek() {
	 				match *c {
	 					c if c.is_digit(10) => { 
	 						n.push(c); 
	 						iter.next();
	 					},
	 					'.' => { 
	 						is_float = true; 
	 						n.push('.'); 
	 						iter.next();
	 					},
	 					_ => break,
	 				};
	 			};
	 			let n = n.into_iter().collect::<String>();
				if is_float {
					result.push(Token::Literal(Literal::Float(n
						.parse::<f64>()
						.expect("Could not parse number")
						)));
				} else {
					result.push(Token::Literal(Literal::Integer(n
						.parse::<i64>()
						.expect("Could not parse number")
						)));
				};
	 		},
	 		'a'..='z' | 'A'..='Z' | '_' => {
	 			let mut s = Vec::new();

	 			s.push(c);

	 			while let Some(c) = iter.peek() {
	 				match *c {
	 					c if c.is_alphabetic() || c == '_' => {
	 						s.push(c);
	 						iter.next();
	 					},
	 					_ => break,
	 				};
	 			};

	 			let s = s.into_iter().collect::<String>();

	 			match &*s {
	 				"if" => result.push(Token::Keyword(Keyword::If)),
	 				"else" => result.push(Token::Keyword(Keyword::Else)),
	 				"elif" => result.push(Token::Keyword(Keyword::Elif)),
	 				"loop" => result.push(Token::Keyword(Keyword::Loop)),
	 				"break" => result.push(Token::Keyword(Keyword::Break)),
	 				"function" => result.push(Token::Keyword(Keyword::Function)),
	 				"return" => result.push(Token::Keyword(Keyword::Return)),
	 				"import" => result.push(Token::Keyword(Keyword::Import)),
	 				"as" => result.push(Token::Keyword(Keyword::As)),
	 				"true" => result.push(Token::Literal(Literal::Bool(true))),
	 				"false" => result.push(Token::Literal(Literal::Bool(false))),
	 				_ => result.push(Token::Literal(Literal::Identifier(s))),
	 			};
	 		},
	 		_ => return Err(format!("Unexpected character: {}", c)),
 		};
 	};

	Ok(result)
}
