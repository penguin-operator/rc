pub enum Token {
	Symbol(String),
	String(String),
	CharStr(String),
	Int(isize),
	Float(isize, usize),
	Semicolon,
	OpenParen, // (
	CloseParen, // )
	OpenBrace, // {
	CloseBrace, // }
	OpenBracket, // [
	CloseBracket, // ]
	Colon,
	DoubleColon,
	Comma,
	Dot,
	Star,
	Slash,
	At,
	Percent,
	Plus,
	PlusPlus,
	Minus,
	MinusMinus,
	Ampersand,
	DoubleAmpersand,
	Pipe,
	DoublePipe,
	Hat,
	DoubleHat,
	Equal,
	DoubleEqual,
	BangEqual,
	Bang,
	EqualArrow,
	Less,
	LessEqual,
	Greater,
	GreaterEqual,
	LeftShift,
	RightShift,
	End,
	From,
	Import,
	Fn,
	Return,
	Yield,
	If,
	ElseIf,
	Else,
	While,
	Break,
	Next,
	Type,
	Extends,
	Struct,
	Union,
	Enum,
}

pub struct LexError {
	pub line: usize,
	pub column: usize,
	pub message: String,
}

pub fn tokenize(src: &str) -> Result<Vec<(Token, usize, usize)>, LexError> {
	let mut tokens = Vec::new();
	let mut chars = src.chars().peekable();
	let mut line = 1;
	let mut column = 1;
	while let Some(&ch) = chars.peek() {
		if ch.is_whitespace() || ch == '\n' || ch == '\r' {
			if ch == '\n' {
				line += 1;
				column = 1;
			} else {
				column += 1;
			}
			chars.next();
		} else if ch.is_alphabetic() || ch == '_' {
			let mut symbol = String::new();
			while let Some(&c) = chars.peek() {
				if c.is_alphanumeric() || c == '_' {
					symbol.push(c);
					column += 1;
					chars.next();
				} else {
					break;
				}
			}
			if symbol == "end" {
				tokens.push((Token::End, line, column));
			} else if symbol == "from" {
				tokens.push((Token::From, line, column));
			} else if symbol == "import" {
				tokens.push((Token::Import, line, column));
			} else if symbol == "fn" {
				tokens.push((Token::Fn, line, column));
			} else if symbol == "return" {
				tokens.push((Token::Return, line, column));
			} else if symbol == "yield" {
				tokens.push((Token::Yield, line, column));
			} else if symbol == "if" {
				tokens.push((Token::If, line, column));
			} else if symbol == "elseif" {
				tokens.push((Token::ElseIf, line, column));
			} else if symbol == "else" {
				tokens.push((Token::Else, line, column));
			} else if symbol == "while" {
				tokens.push((Token::While, line, column));
			} else if symbol == "break" {
				tokens.push((Token::Break, line, column));
			} else if symbol == "continue" {
				tokens.push((Token::Next, line, column));
			} else if symbol == "type" {
				tokens.push((Token::Type, line, column));
			} else if symbol == "extends" {
				tokens.push((Token::Extends, line, column));
			} else if symbol == "struct" {
				tokens.push((Token::Struct, line, column));
			} else if symbol == "union" {
				tokens.push((Token::Union, line, column));
			} else if symbol == "enum" {
				tokens.push((Token::Enum, line, column));
			} else {
				tokens.push((Token::Symbol(symbol), line, column));
			}
		} else if ch == '\'' {
			column += 1;
			chars.next();
			let mut char_str = String::new();
			while let Some(&c) = chars.peek() {
				column += 1;
				if c != '\'' {
					char_str.push(c);
					chars.next();
				} else {
					break;
				}
			}
			column += 1;
			chars.next();
			if chars.peek() == None {
				return Err(LexError {
					line,
					column,
					message: String::from("Unterminated string literal"),
				});
			}
			tokens.push((Token::CharStr(char_str), line, column));
		} else if ch == '"' {
			column += 1;
			chars.next();
			let mut char_str = String::new();
			while let Some(&c) = chars.peek() {
				column += 1;
				if c != '\"' {
					char_str.push(c);
					chars.next();
				} else {
					break;
				}
			}
			column += 1;
			chars.next();
			if chars.peek() == None {
				return Err(LexError {
					line,
					column,
					message: String::from("Unterminated string literal"),
				});
			}
			tokens.push((Token::String(char_str), line, column));
		} else if ch.is_numeric() {
			let mut int_str = if tokens.last().unwrap().0 == Token::Minus {
				tokens.pop();
				String::from("-")
			} else {
				String::new()
			};
			let mut frac = 0isize;
			let mut is_float = false;
			while let Some(&c) = chars.peek() {
				if c.is_numeric() {
					int_str.push(c);
					column += 1;
					chars.next();
				} else if c == '.' {
					column += 1;
					chars.next();
					is_float = true;
					while let Some(&c2) = chars.peek() {
						if c2.is_numeric() {
							int_str.push(c2);
							frac += 1;
							column += 1;
							chars.next();
						} else {
							break;
						}
					}
					break;
				} else {
					break;
				}
			}
			let mut int_value = int_str.parse::<isize>().unwrap();
			if is_float {
				while int_value % 10 == 0 && frac > 0 {
					int_value /= 10;
					frac -= 1;
				}
				tokens.push((Token::Float(int_value, frac as usize), line, column));
			} else {
				tokens.push((Token::Int(int_value), line, column));
			}
		} else {
			match ch {
				';' => {
					column += 1;
					tokens.push((Token::Semicolon, line, column));
					chars.next();
				},
				'(' => {
					column += 1;
					tokens.push((Token::OpenParen, line, column));
					chars.next();
				},
				')' => {
					column += 1;
					tokens.push((Token::CloseParen, line, column));
					chars.next();
				},
				'{' => {
					column += 1;
					tokens.push((Token::OpenBrace, line, column));
					chars.next();
				},
				'}' => {
					column += 1;
					tokens.push((Token::CloseBrace, line, column));
					chars.next();
				},
				'[' => {
					column += 1;
					tokens.push((Token::OpenBracket, line, column));
					chars.next();
				},
				']' => {
					column += 1;
					tokens.push((Token::CloseBracket, line, column));
					chars.next();
				},
				',' => {
					column += 1;
					tokens.push((Token::Comma, line, column));
					chars.next();
				},
				'.' => {
					column += 1;
					tokens.push((Token::Dot, line, column));
					chars.next();
				},
				'*' => {
					column += 1;
					tokens.push((Token::Star, line, column));
					chars.next();
				},
				'/' => {
					column += 1;
					tokens.push((Token::Slash, line, column));
					chars.next();
				},
				'%' => {
					column += 1;
					tokens.push((Token::Percent, line, column));
					chars.next();
				},
				'@' => {
					column += 1;
					tokens.push((Token::At, line, column));
					chars.next();
				},
				':' => {
					column += 1;
					chars.next();
					if let Some(&':') = chars.peek() {
						column += 1;
						tokens.push((Token::DoubleColon, line, column));
						chars.next();
					} else {
						tokens.push((Token::Colon, line, column));
					}
				},
				'!' => {
					column += 1;
					chars.next();
					if let Some(&'=') = chars.peek() {
						column += 1;
						tokens.push((Token::BangEqual, line, column));
						chars.next();
					} else {
						tokens.push((Token::Bang, line, column));
					}
				},
				'=' => {
					column += 1;
					chars.next();
					if let Some(&'=') = chars.peek() {
						column += 1;
						tokens.push((Token::DoubleEqual, line, column));
						chars.next();
					} else if let Some(&'>') = chars.peek() {
						column += 1;
						tokens.push((Token::EqualArrow, line, column));
						chars.next();
					} else {
						tokens.push((Token::Equal, line, column));
					}
				},
				'<' => {
					column += 1;
					chars.next();
					if let Some(&'=') = chars.peek() {
						column += 1;
						tokens.push((Token::LessEqual, line, column));
						chars.next();
					} else if let Some(&'<') = chars.peek() {
						column += 1;
						tokens.push((Token::LeftShift, line, column));
						chars.next();
					} else {
						tokens.push((Token::Less, line, column));
					}
				},
				'>' => {
					column += 1;
					chars.next();
					if let Some(&'=') = chars.peek() {
						column += 1;
						tokens.push((Token::GreaterEqual, line, column));
						chars.next();
					} else if let Some(&'>') = chars.peek() {
						column += 1;
						tokens.push((Token::RightShift, line, column));
						chars.next();
					} else {
						tokens.push((Token::Greater, line, column));
					}
				},
				'+' => {
					column += 1;
					chars.next();
					if let Some(&'+') = chars.peek() {
						column += 1;
						tokens.push((Token::PlusPlus, line, column));
						chars.next();
					} else {
						tokens.push((Token::Plus, line, column));
					}
				},
				'-' => {
					column += 1;
					chars.next();
					if let Some(&'-') = chars.peek() {
						column += 1;
						tokens.push((Token::MinusMinus, line, column));
						chars.next();
					} else {
						tokens.push((Token::Minus, line, column));
					}
				},
				'&' => {
					column += 1;
					chars.next();
					if let Some(&'&') = chars.peek() {
						column += 1;
						tokens.push((Token::DoubleAmpersand, line, column));
						chars.next();
					} else {
						tokens.push((Token::Ampersand, line, column));
					}
				},
				'|' => {
					column += 1;
					chars.next();
					if let Some(&'|') = chars.peek() {
						column += 1;
						tokens.push((Token::DoublePipe, line, column));
						chars.next();
					} else {
						tokens.push((Token::Pipe, line, column));
					}
				},
				'^' => {
					column += 1;
					chars.next();
					if let Some(&'^') = chars.peek() {
						column += 1;
						tokens.push((Token::DoubleHat, line, column));
						chars.next();
					} else {
						tokens.push((Token::Hat, line, column));
					}
				},
				_ => return Err(LexError {
					line,
					column,
					message: format!("Unexpected character: {}", ch),
				}),
				
			}
		}
	}
	Ok(tokens)
}

impl std::fmt::Debug for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Token::Symbol(s) => write!(f, "{}", s),
			Token::String(s) => write!(f, "\"{}\"", s),
			Token::CharStr(s) => write!(f, "'{}'", s),
			Token::Int(i) => write!(f, "{}", i),
			Token::Float(i, frac) => {
				let mut fstr = i.to_string();
				if *frac > fstr.len() {
					write!(f, "0.{}{}", "0".repeat(*frac - fstr.len()), fstr)
				} else {
					fstr.insert(fstr.len() - *frac, '.');
					write!(f, "{}", fstr)
				}
			},
			Token::Semicolon => write!(f, ";"),
			Token::OpenParen => write!(f, "("),
			Token::CloseParen => write!(f, ")"),
			Token::OpenBrace => write!(f, "{{"),
			Token::CloseBrace => write!(f, "}}"),
			Token::OpenBracket => write!(f, "["),
			Token::CloseBracket => write!(f, "]"),
			Token::Colon => write!(f, ":"),
			Token::DoubleColon => write!(f, "::"),
			Token::Comma => write!(f, ","),
			Token::Dot => write!(f, "."),
			Token::Star => write!(f, "*"),
			Token::Slash => write!(f, "/"),
			Token::At => write!(f, "@"),
			Token::Percent => write!(f, "%"),
			Token::Plus => write!(f, "+"),
			Token::PlusPlus => write!(f, "++"),
			Token::Minus => write!(f, "-"),
			Token::MinusMinus => write!(f, "--"),
			Token::Ampersand => write!(f, "&"),
			Token::DoubleAmpersand => write!(f, "&&"),
			Token::Pipe => write!(f, "|"),
			Token::DoublePipe => write!(f, "||"),
			Token::Hat => write!(f, "^"),
			Token::DoubleHat => write!(f, "^^"),
			Token::Equal => write!(f, "="),
			Token::DoubleEqual => write!(f, "=="),
			Token::BangEqual => write!(f, "!="),
			Token::Bang => write!(f, "!"),
			Token::EqualArrow => write!(f, "=>"),
			Token::Less => write!(f, "<"),
			Token::LessEqual => write!(f, "<="),
			Token::Greater => write!(f, ">"),
			Token::GreaterEqual => write!(f, ">="),
			Token::LeftShift => write!(f, "<<"),
			Token::RightShift => write!(f, ">>"),
			Token::End => write!(f, "end"),
			Token::From => write!(f, "from"),
			Token::Import => write!(f, "import"),
			Token::Fn => write!(f, "fn"),
			Token::Return => write!(f, "return"),
			Token::Yield => write!(f, "yield"),
			Token::If => write!(f, "if"),
			Token::ElseIf => write!(f, "elseif"),
			Token::Else => write!(f, "else"),
			Token::While => write!(f, "while"),
			Token::Break => write!(f, "break"),
			Token::Next => write!(f, "continue"),
			Token::Type => write!(f, "type"),
			Token::Extends => write!(f, "extends"),
			Token::Struct => write!(f, "struct"),
			Token::Union => write!(f, "union"),
			Token::Enum => write!(f, "enum"),
		}
	}
}

impl core::cmp::PartialEq for Token {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Token::Symbol(s1), Token::Symbol(s2)) => s1 == s2,
			(Token::String(s1), Token::String(s2)) => s1 == s2,
			(Token::CharStr(s1), Token::CharStr(s2)) => s1 == s2,
			(Token::Int(i1), Token::Int(i2)) => i1 == i2,
			(Token::Float(i1, f1), Token::Float(i2, f2)) => i1 == i2 && f1 == f2,
			(Token::Semicolon, Token::Semicolon) => true,
			(Token::OpenParen, Token::OpenParen) => true,
			(Token::CloseParen, Token::CloseParen) => true,
			(Token::OpenBrace, Token::OpenBrace) => true,
			(Token::CloseBrace, Token::CloseBrace) => true,
			(Token::OpenBracket, Token::OpenBracket) => true,
			(Token::CloseBracket, Token::CloseBracket) => true,
			(Token::Colon, Token::Colon) => true,
			(Token::DoubleColon, Token::DoubleColon) => true,
			(Token::Comma, Token::Comma) => true,
			(Token::Dot, Token::Dot) => true,
			(Token::Star, Token::Star) => true,
			(Token::Slash, Token::Slash) => true,
			(Token::At, Token::At) => true,
			(Token::Percent, Token::Percent) => true,
			(Token::Plus, Token::Plus) => true,
			(Token::PlusPlus, Token::PlusPlus) => true,
			(Token::Minus, Token::Minus) => true,
			(Token::MinusMinus, Token::MinusMinus) => true,
			(Token::Ampersand, Token::Ampersand) => true,
			(Token::DoubleAmpersand, Token::DoubleAmpersand) => true,
			(Token::Pipe, Token::Pipe) => true,
			(Token::DoublePipe, Token::DoublePipe) => true,
			(Token::Hat, Token::Hat) => true,
			(Token::DoubleHat, Token::DoubleHat) => true,
			(Token::Equal, Token::Equal) => true,
			(Token::DoubleEqual, Token::DoubleEqual) => true,
			(Token::BangEqual, Token::BangEqual) => true,
			(Token::Bang, Token::Bang) => true,
			(Token::EqualArrow, Token::EqualArrow) => true,
			(Token::Less, Token::Less) => true,
			(Token::LessEqual, Token::LessEqual) => true,
			(Token::Greater, Token::Greater) => true,
			(Token::GreaterEqual, Token::GreaterEqual) => true,
			(Token::LeftShift, Token::LeftShift) => true,
			(Token::RightShift, Token::RightShift) => true,
			(Token::End, Token::End) => true,
			(Token::From, Token::From) => true,
			(Token::Import, Token::Import) => true,
			(Token::Fn, Token::Fn) => true,
			(Token::Return, Token::Return) => true,
			(Token::Yield, Token::Yield) => true,
			(Token::If, Token::If) => true,
			(Token::ElseIf, Token::ElseIf) => true,
			(Token::Else, Token::Else) => true,
			(Token::While, Token::While) => true,
			(Token::Break, Token::Break) => true,
			(Token::Next, Token::Next) => true,
			(Token::Type, Token::Type) => true,
			(Token::Extends, Token::Extends) => true,
			(Token::Struct, Token::Struct) => true,
			(Token::Union, Token::Union) => true,
			(Token::Enum, Token::Enum) => true,
			_ => false,
		}
	}
}

impl std::fmt::Display for LexError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:{}: {}", self.line, self.column, self.message)
	}
}
