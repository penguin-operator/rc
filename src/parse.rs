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

pub fn parse(file: &str, src: Vec<Token>) -> Result<Module, BuildError> {
	unimplemented!()
}
