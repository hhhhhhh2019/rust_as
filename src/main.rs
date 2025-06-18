#![cfg_attr(debug_assertions, allow(dead_code, unused))]

mod token;
mod expr;
mod parser;
mod asm;
mod minipre;

use logos::Logos;
use token::Token;

use expr::{Expr, ExprKind};
use parser::{reduce, Operation};
use asm::{opcode, register, datatype, get_size};

use minipre::{process_str, Context};
use regex::Regex;


fn token_value(tok: Token) -> ExprKind {
	match tok {
		Token::Number(n) => ExprKind::Number(n),
		Token::Reg(n) => ExprKind::Reg(register(n)),
		Token::Label(n) => ExprKind::Label(n),
		Token::Id(n) => ExprKind::Id(n),
		Token::IName(n) => {
			if n.ends_with("B") {
				ExprKind::IName(opcode(n.strip_suffix("B").unwrap()), 0)
			}
			else if n.ends_with("S")  {
				ExprKind::IName(opcode(n.strip_suffix("S").unwrap()), 1)
			}
			else if n.ends_with("I") {
				ExprKind::IName(opcode(n.strip_suffix("I").unwrap()), 2)
			}
			else if n.ends_with("L") {
				ExprKind::IName(opcode(n.strip_suffix("L").unwrap()), 3)
			} else {
				ExprKind::IName(opcode(n), 3)
			}
		},
		Token::DataType(n) => ExprKind::DType(datatype(n)),
		_ => ExprKind::None,
	}
}


fn read_file(filename: String) -> String {
	let mut file = std::fs::read_to_string(filename).expect("file {filename} not found");

	let regex = Regex::new("#include \".+\"").unwrap();
	let name_regex = Regex::new("\".+\"").unwrap();

	while let Some(c) = regex.captures(&file) {
		let c = c.get(0).unwrap();

		// TODO: fix file paths
		let include_filename = name_regex
			.captures(c.as_str())
			.unwrap()
			.get(0)
			.map_or("", |m| m.as_str().strip_prefix("\"").unwrap().strip_suffix("\"").unwrap());

		let included = read_file(include_filename.to_string());

		let start = c.start();
		let end = c.end();

		file.drain(start..end);
		file.insert_str(start, &included);
	}

	file
}


fn main() {
	let mut ctx = Context::new();

	let inp = read_file(std::env::args().nth(1).expect("expected input filename"));

	let inp = process_str(
		inp.as_str(),
		&mut ctx
	).unwrap();

	dbg!(&inp);

	let mut tokens = Token::lexer(&inp)
		.spanned()
		.map(|(t, s)| {
			match t {
				Ok(tok) => (tok.clone(), Expr{
					span: s,
					kind: token_value(tok),
					..Default::default()
				}),
				Err(_) => unimplemented!()
			}
		})
		.peekable();

	let start = tokens.next().unwrap_or((
		Token::EOI,
		Expr{..Default::default()},
	));

	let mut stack: Vec<Token> = vec![start.0];
	let mut val_stack: Vec<Expr> = vec![start.1];

	loop {
		// dbg!(&stack);
		// dbg!(&val_stack);

		match reduce(&stack, tokens.peek().unwrap_or(&(
			Token::EOI,
			Expr{..Default::default()},
		)).0.clone()) {
			Operation::NOMATCH => break,
			Operation::SHIFT(expected) => {
				let next = tokens.next().unwrap_or((
					Token::EOI,
					Expr{..Default::default()},
				));
				let mut find = false;
				for i in expected {
					if matches!(find, i) {
						find = true;
						break;
					}
				}
				if !find {
					unimplemented!();
				}
				stack.push(next.0);
				val_stack.push(next.1);
			},
			Operation::REDUCE(n, action) => {
				stack.drain(stack.len() - n..);
				let (t, v) = action(
					val_stack.drain(val_stack.len() - n..).collect(),
				);
				stack.push(t);
				val_stack.push(v);
			},
		}
	}

	// dbg!(&stack);
	// dbg!(&val_stack);

	for i in &mut val_stack {
		i.size = get_size(i);
	}

	let mut offset = 0;

	for i in &mut val_stack {
		i.update_offset(offset);
		offset += i.size;
		offset = (offset + 3) / 4 * 4;
	}

	// dbg!(&val_stack);

	let mut output: Vec<u8> = vec![];
	let labels = val_stack.iter().cloned()
		.filter(|e| matches!(e.kind, ExprKind::Label(_)))
		.collect();

	for i in val_stack {
		let mut bytes = i.to_bytes(&labels);
		for _ in 0..(bytes.len() + 3) / 4 * 4 - bytes.len() {
			bytes.push(0);
		}
		output.extend(bytes);
	}


	let _ = std::fs::write(std::env::args().nth(2).expect("expected output filename"), output);
}
