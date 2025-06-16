#![allow(dead_code, unused)]

mod token;
mod expr;
mod parser;
mod asm;

use logos::Logos;
use token::Token;

use expr::{Expr, ExprKind};
use parser::{reduce, Operation};
use asm::{opcode, register, datatype};


fn token_value(tok: Token) -> ExprKind {
	match tok {
		Token::Number(n) => ExprKind::Number(n),
		Token::Reg(n) => ExprKind::Reg(register(n)),
		Token::Label(n) => ExprKind::Label(n),
		Token::Id(n) => ExprKind::Id(n),
		Token::IName(n) => {
			if n.ends_with("b") {
				ExprKind::IName(opcode(n.strip_suffix("b").unwrap()), 0)
			}
			else if n.ends_with("s") {
				ExprKind::IName(opcode(n.strip_suffix("s").unwrap()), 1)
			}
			else if n.ends_with("i") {
				ExprKind::IName(opcode(n.strip_suffix("i").unwrap()), 2)
			}
			else if n.ends_with("l") {
				ExprKind::IName(opcode(n.strip_suffix("l").unwrap()), 3)
			} else {
				ExprKind::IName(opcode(n), 3)
			}
		},
		Token::DataType(n) => ExprKind::DType(datatype(n)),
		_ => ExprKind::None,
	}
}


fn main() {
	let inp = std::fs::read_to_string("input.S").unwrap();

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
		.peekable()
		;

	let start = tokens.next().unwrap_or((
		Token::EOI,
		Expr{..Default::default()},
	));

	let mut stack: Vec<Token> = vec![start.0];
	let mut val_stack: Vec<Expr> = vec![start.1];

	loop {
		dbg!(&stack);
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
				let (t, v) = action(
					stack.drain(stack.len() - n..).collect(),
					val_stack.drain(val_stack.len() - n..).collect(),
				);
				stack.push(t);
				val_stack.push(v);
			},
		}
	}

	dbg!(&stack);
	dbg!(&val_stack);
}
