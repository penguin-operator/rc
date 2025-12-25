use std::fmt::Debug;

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
	Continue,
	Type,
	Struct,
	Union,
	Enum,
	Extern,
	Inline,
}

pub struct LexError {
	pub line: usize,
	pub column: usize,
	pub message: String,
}

pub fn tokenize(src: &str) -> Result<Vec<Token>, LexError> {
	let mut tokens: Vec<Token> = Vec::new();
	let mut chars = src.chars().peekable();
	while let Some(&ch) = chars.peek() {
		if ch.is_whitespace() || ch == '\n' || ch == '\r' {
			chars.next();
		} else if ch.is_alphabetic() || ch == '_' {
			let mut symbol = String::new();
			while let Some(&c) = chars.peek() {
				if c.is_alphanumeric() || c == '_' {
					symbol.push(c);
					chars.next();
				} else {
					break;
				}
			}
			if symbol == "end" {
				tokens.push(Token::End);
			} else if symbol == "from" {
				tokens.push(Token::From);
			} else if symbol == "import" {
				tokens.push(Token::Import);
			} else if symbol == "fn" {
				tokens.push(Token::Fn);
			} else if symbol == "return" {
				tokens.push(Token::Return);
			} else if symbol == "yield" {
				tokens.push(Token::Yield);
			} else if symbol == "if" {
				tokens.push(Token::If);
			} else if symbol == "elseif" {
				tokens.push(Token::ElseIf);
			} else if symbol == "else" {
				tokens.push(Token::Else);
			} else if symbol == "while" {
				tokens.push(Token::While);
			} else if symbol == "break" {
				tokens.push(Token::Break);
			} else if symbol == "continue" {
				tokens.push(Token::Continue);
			} else if symbol == "type" {
				tokens.push(Token::Type);
			} else if symbol == "struct" {
				tokens.push(Token::Struct);
			} else if symbol == "union" {
				tokens.push(Token::Union);
			} else if symbol == "enum" {
				tokens.push(Token::Enum);
			} else if symbol == "extern" {
				tokens.push(Token::Extern);
			} else if symbol == "inline" {
				tokens.push(Token::Inline);
			} else {
				tokens.push(Token::Symbol(symbol));
			}
		} else if ch == '\'' {
			chars.next();
			let mut char_str = String::new();
			while let Some(&c) = chars.peek() {
				if c != '\'' {
					char_str.push(c);
					chars.next();
				} else {
					break;
				}
			}
			chars.next();
			tokens.push(Token::CharStr(char_str));
		} else if ch == '"' {
			chars.next();
			let mut char_str = String::new();
			while let Some(&c) = chars.peek() {
				if c != '\"' {
					char_str.push(c);
					chars.next();
				} else {
					break;
				}
			}
			chars.next();
			tokens.push(Token::String(char_str));
		} else if ch.is_numeric() {
			let mut int_str = if *tokens.last().unwrap() == Token::Minus {
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
					chars.next();
				} else if c == '.' {
					chars.next();
					is_float = true;
					while let Some(&c2) = chars.peek() {
						if c2.is_numeric() {
							int_str.push(c2);
							frac += 1;
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
				tokens.push(Token::Float(int_value, frac as usize));
			} else {
				tokens.push(Token::Int(int_value));
			}
		} else {
			match ch {
				';' => {
					tokens.push(Token::Semicolon);
					chars.next();
				},
				'(' => {
					tokens.push(Token::OpenParen);
					chars.next();
				},
				')' => {
					tokens.push(Token::CloseParen);
					chars.next();
				},
				'{' => {
					tokens.push(Token::OpenBrace);
					chars.next();
				},
				'}' => {
					tokens.push(Token::CloseBrace);
					chars.next();
				},
				'[' => {
					tokens.push(Token::OpenBracket);
					chars.next();
				},
				']' => {
					tokens.push(Token::CloseBracket);
					chars.next();
				},
				',' => {
					tokens.push(Token::Comma);
					chars.next();
				},
				'.' => {
					tokens.push(Token::Dot);
					chars.next();
				},
				'*' => {
					tokens.push(Token::Star);
					chars.next();
				},
				'/' => {
					tokens.push(Token::Slash);
					chars.next();
				},
				'%' => {
					tokens.push(Token::Percent);
					chars.next();
				},
				'@' => {
					tokens.push(Token::At);
					chars.next();
				},
				':' => {
					chars.next();
					if let Some(&':') = chars.peek() {
						tokens.push(Token::DoubleColon);
						chars.next();
					} else {
						tokens.push(Token::Colon);
					}
				},
				'!' => {
					chars.next();
					if let Some(&'=') = chars.peek() {
						tokens.push(Token::BangEqual);
						chars.next();
					} else {
						tokens.push(Token::Bang);
					}
				},
				'=' => {
					chars.next();
					if let Some(&'=') = chars.peek() {
						tokens.push(Token::DoubleEqual);
						chars.next();
					} else if let Some(&'>') = chars.peek() {
						tokens.push(Token::EqualArrow);
						chars.next();
					} else {
						tokens.push(Token::Equal);
					}
				},
				'<' => {
					chars.next();
					if let Some(&'=') = chars.peek() {
						tokens.push(Token::LessEqual);
						chars.next();
					} else if let Some(&'<') = chars.peek() {
						tokens.push(Token::LeftShift);
						chars.next();
					} else {
						tokens.push(Token::Less);
					}
				},
				'>' => {
					chars.next();
					if let Some(&'=') = chars.peek() {
						tokens.push(Token::GreaterEqual);
						chars.next();
					} else if let Some(&'>') = chars.peek() {
						tokens.push(Token::RightShift);
						chars.next();
					} else {
						tokens.push(Token::Greater);
					}
				},
				'+' => {
					chars.next();
					if let Some(&'+') = chars.peek() {
						tokens.push(Token::PlusPlus);
						chars.next();
					} else {
						tokens.push(Token::Plus);
					}
				},
				'-' => {
					chars.next();
					if let Some(&'-') = chars.peek() {
						tokens.push(Token::MinusMinus);
						chars.next();
					} else {
						tokens.push(Token::Minus);
					}
				},
				'&' => {
					chars.next();
					if let Some(&'&') = chars.peek() {
						tokens.push(Token::DoubleAmpersand);
						chars.next();
					} else {
						tokens.push(Token::Ampersand);
					}
				},
				'|' => {
					chars.next();
					if let Some(&'|') = chars.peek() {
						tokens.push(Token::DoublePipe);
						chars.next();
					} else {
						tokens.push(Token::Pipe);
					}
				},
				'^' => {
					chars.next();
					if let Some(&'^') = chars.peek() {
						tokens.push(Token::DoubleHat);
						chars.next();
					} else {
						tokens.push(Token::Hat);
					}
				},
				_ => {
					let mut line = 1;
					let mut column = 1;
					for c in src.chars() {
						if c == '\n' {
							line += 1;
							column = 1;
						} else {
							column += 1;
						}
					}
					return Err(LexError {
						line,
						column,
						message: format!("Unexpected character: {}", ch),
					});
				},
				
			}
		}
	}
	Ok(tokens)
}

impl Debug for Token {
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
			Token::Continue => write!(f, "continue"),
			Token::Type => write!(f, "type"),
			Token::Struct => write!(f, "struct"),
			Token::Union => write!(f, "union"),
			Token::Enum => write!(f, "enum"),
			Token::Extern => write!(f, "extern"),
			Token::Inline => write!(f, "inline"),
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
			(Token::Continue, Token::Continue) => true,
			(Token::Type, Token::Type) => true,
			(Token::Struct, Token::Struct) => true,
			(Token::Union, Token::Union) => true,
			(Token::Enum, Token::Enum) => true,
			(Token::Extern, Token::Extern) => true,
			(Token::Inline, Token::Inline) => true,
			_ => false,
		}
	}
}

impl std::fmt::Display for LexError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:{}: {}", self.line, self.column, self.message)
	}
}
