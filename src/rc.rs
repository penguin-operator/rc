use std::{
	env::{
		args,
		current_dir,
	},
	fs, process::exit,
};

use crate::ast::Module;

pub mod ast;
pub mod lex;
pub mod parse;

pub fn main() {
	let mut input: Vec<String> = vec![];
	let mut output: String = current_dir()
		.unwrap()
		.as_path()
		.file_name()
		.unwrap()
		.to_str()
		.unwrap()
		.into();
	let mut modules: Vec<Module> = Vec::new();

	for arg in args().skip(1) {
		if arg.starts_with("-") {
			match arg.chars().nth(1) {
				Some('o') => output = arg[2..].into(),
				Some(_) | None => {},
			}
		} else {
			input.push(arg);
		}
	}

	for (file, src) in input.iter().map(|file| {
		match fs::read(file) {
			Ok(code) => match String::from_utf8(code) {
				Ok(code) => (file.clone(), code.clone()),
				Err(err) => {
					eprintln!("\x1b[1;31merror\x1b[0m could not load {}: {}", file, err);
					exit(1)
				},
			},
			Err(err) => {
				eprintln!("\x1b[1;31merror\x1b[0m could not load {}: {}", file, err);
				exit(1)
			},
		}
	}) {
		let tokens = match lex::tokenize(&src) {
			Ok(tokens) => tokens,
			Err(err) => {
				eprintln!("\x1b[1;31merror\x1b[0m {}:{}", file, err);
				exit(1)
			},
		};
		let module = match parse::parse(&file, tokens) {
			Ok(module) => module,
			Err(err) => {
				eprintln!("\x1b[1;31merror\x1b[0m {}:{}", file, err);
				exit(1)
			},
		};
	};
}
