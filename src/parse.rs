use crate::{
	ast::Module,
	lex::Token,
};

pub struct BuildError {
	pub line: usize,
	pub column: usize,
	pub message: String,
}

impl std::fmt::Display for BuildError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}:{}: {}", self.line, self.column, self.message)
	}
}

pub fn parse(file: &str, src: Vec<(Token, usize, usize)>) -> Result<Module, BuildError> {
	let mut module = Module { path: file.into(), imports: vec![], types: vec![], symbols: vec![] };
	let mut stream = src.iter().peekable();
	while let Some((token, line, column)) = stream.peek() {
		if token == &Token::From {
			stream.next();
			if let Some((Token::String(path), line, column)) = stream.next() {
				match stream.next() {
					Some((Token::Import, line, column)) => {
						let mut imports: Vec<String> = vec![];
						if let Some((Token::Symbol(name), _, _)) = stream.next() {
							imports.push(name.clone());
						} else {
							return Err(BuildError {
								line: *line,
								column: *column,
								message: "expected symbol after 'import'".into(),
							});
						}
						while let Some((Token::Comma, line, column)) = stream.peek() {
							stream.next();
							if let Some((Token::Symbol(name), _, _)) = stream.peek() {
								stream.next();
								imports.push(name.clone());
							}
						}
						module.imports.push((path.clone(), imports));
					},
					_ => return Err(BuildError {
						line: *line,
						column: *column,
						message: "expected 'import'".into()
					}),
				}
			} else {
				return Err(BuildError {
					line: *line,
					column: *column,
					message: "expected module path after 'from'".into(),
				});
			}
		} else if let Token::Symbol(name) = token {
			let typ_ = match parse_type(&mut stream) {
				Ok(typ_) => typ_,
				Err(err) => return Err(err),
			};
		} else {
			return Err(BuildError {
				line: *line,
				column: *column,
				message: format!("unexpected token '{:?}'", token),
			});
		}
	}
	Ok(module)
}

pub fn parse_type(
	stream: &mut std::iter::Peekable<std::slice::Iter<(Token, usize, usize)>>
) -> Result<crate::ast::Type, BuildError> {
	
	unimplemented!()
}
