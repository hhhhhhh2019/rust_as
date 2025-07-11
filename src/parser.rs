use super::token::Token;
use super::expr::{Expr, ExprKind};

use Token::*;


pub enum Operation<'a> {
	NOMATCH,
	SHIFT(Vec<Token<'a>>),
	REDUCE(usize, &'a dyn Fn(Vec<Expr<'a>>) -> (Token<'a>, Expr<'a>)),
}

pub fn reduce<'a>(stack: &Vec<Token<'a>>, lookahead: Token<'a>) -> Operation<'a> {
	if stack.len() >= 6 { match (
		&stack[stack.len() - 6],
		&stack[stack.len() - 5],
		&stack[stack.len() - 4],
		&stack[stack.len() - 3],
		&stack[stack.len() - 2],
		&stack[stack.len() - 1],
		&lookahead
	) {
		(IName(_), E, COMMA, E, COMMA, E, _) => return
			Operation::REDUCE(6, &|vals| {
				if let ExprKind::IName(op, size) = vals[0].kind {
					(Instr, Expr{
						kind: ExprKind::Instruction(op, size, vec![
							vals[1].clone(), vals[3].clone(), vals[5].clone()
						]),
						span: vals[0].span.start..vals[5].span.end,
						..Default::default()
					})
				} else {unreachable!()}}),
		_ => {}
	}}

	if stack.len() >= 5 { match (
		&stack[stack.len() - 5],
		&stack[stack.len() - 4],
		&stack[stack.len() - 3],
		&stack[stack.len() - 2],
		&stack[stack.len() - 1],
		&lookahead
	) {
		(IName(_), E, COMMA, E, COMMA, _) => return
			Operation::SHIFT(vec![Reg(""), LBR, Number(0), TILDA, Id("")]),
		_ => {}
	}}

	if stack.len() >= 4 { match (
		&stack[stack.len() - 4],
		&stack[stack.len() - 3],
		&stack[stack.len() - 2],
		&stack[stack.len() - 1],
		&lookahead
	) {
		(IName(_), E, COMMA, E, COMMA) => return
			Operation::SHIFT(vec![COMMA]),

		(IName(_), E, COMMA, E, _) => return
			Operation::REDUCE(4, &|vals| {
				if let ExprKind::IName(op, size) = vals[0].kind {
					(Instr, Expr{
						kind: ExprKind::Instruction(op, size, vec![
							vals[1].clone(), vals[3].clone()
						]),
						span: vals[0].span.start..vals[3].span.end,
						..Default::default()
					})
				} else {unreachable!()}}),
		_ => {}
	}}

	if stack.len() >= 3 { match (
		&stack[stack.len() - 3],
		&stack[stack.len() - 2],
		&stack[stack.len() - 1],
		&lookahead
	) {
		(IName(_), E, COMMA, _) => return
			Operation::SHIFT(vec![Reg(""), LBR, Number(0), TILDA, Id("")]),

		(Vals, COMMA, E, _) => return
			Operation::REDUCE(3, &|vals| {
				if let ExprKind::Vals(arr) = &vals[0].kind {
					let mut arr = arr.clone();
					arr.push(vals[2].clone());
					(Vals, Expr{
						kind: ExprKind::Vals(arr),
						span: vals[0].span.start..vals[2].span.end,
						..Default::default()
					})
				} else {unreachable!()}
			}),

		(DataType(_), Vals, COMMA, _) => return
			Operation::SHIFT(vec![LBR, Number(0), TILDA, Id("")]),

		(E1, PIPE, E1, _) => return
			Operation::REDUCE(3, &|vals| {
				(E1, Expr{
					kind: ExprKind::Or(Box::new(vals[0].clone()), Box::new(vals[2].clone())),
					span: vals[0].span.start..vals[2].span.start,
					..Default::default()
				})
			}),

		(E2, CARET, E2, _) => return
			Operation::REDUCE(3, &|vals| {
				(E2, Expr{
					kind: ExprKind::Xor(Box::new(vals[0].clone()), Box::new(vals[2].clone())),
					span: vals[0].span.start..vals[2].span.start,
					..Default::default()
				})
			}),

		(E3, AMPERSAND, E3, _) => return
			Operation::REDUCE(3, &|vals| {
				(E3, Expr{
					kind: ExprKind::And(Box::new(vals[0].clone()), Box::new(vals[2].clone())),
					span: vals[0].span.start..vals[2].span.start,
					..Default::default()
				})
			}),

		(E4, LSHIFT, E4, _) => return
			Operation::REDUCE(3, &|vals| {
				(E4, Expr{
					kind: ExprKind::Lsh(Box::new(vals[0].clone()), Box::new(vals[2].clone())),
					span: vals[0].span.start..vals[2].span.start,
					..Default::default()
				})
			}),

		(E4, RSHIFT, E4, _) => return
			Operation::REDUCE(3, &|vals| {
				(E4, Expr{
					kind: ExprKind::Rsh(Box::new(vals[0].clone()), Box::new(vals[2].clone())),
					span: vals[0].span.start..vals[2].span.start,
					..Default::default()
				})
			}),

		(E5, PLUS, E5, _) => return
			Operation::REDUCE(3, &|vals| {
				(E5, Expr{
					kind: ExprKind::Sum(Box::new(vals[0].clone()), Box::new(vals[2].clone())),
					span: vals[0].span.start..vals[2].span.start,
					..Default::default()
				})
			}),

		(E5, MINUS, E5, _) => return
			Operation::REDUCE(3, &|vals| {
				(E5, Expr{
					kind: ExprKind::Sub(Box::new(vals[0].clone()), Box::new(vals[2].clone())),
					span: vals[0].span.start..vals[2].span.start,
					..Default::default()
				})
			}),

		(E6, PERCENT, E6, _) => return
			Operation::REDUCE(3, &|vals| {
				(E6, Expr{
					kind: ExprKind::Mod(Box::new(vals[0].clone()), Box::new(vals[2].clone())),
					span: vals[0].span.start..vals[2].span.start,
					..Default::default()
				})
			}),

		(E6, STAR, E6, _) => return
			Operation::REDUCE(3, &|vals| {
				(E6, Expr{
					kind: ExprKind::Mul(Box::new(vals[0].clone()), Box::new(vals[2].clone())),
					span: vals[0].span.start..vals[2].span.start,
					..Default::default()
				})
			}),

		(E6, SLASH, E6, _) => return
			Operation::REDUCE(3, &|vals| {
				(E6, Expr{
					kind: ExprKind::Div(Box::new(vals[0].clone()), Box::new(vals[2].clone())),
					span: vals[0].span.start..vals[2].span.start,
					..Default::default()
				})
			}),

		(LBR, E1, RBR, _) => return
			Operation::REDUCE(3, &|vals| {
				(E8, Expr{
					kind: vals[1].kind.clone(),
					span: vals[0].span.start..vals[2].span.start,
					..Default::default()
				})
			}),

		_ => {}
	}}

	if stack.len() >= 2 { match (
		&stack[stack.len() - 2],
		&stack[stack.len() - 1],
		&lookahead
	) {
		(IName(_), E, COMMA) => return
			Operation::SHIFT(vec![COMMA]),

		(E1, PIPE, _) |
		(E2, CARET, _) |
		(E3, AMPERSAND, _) |
		(E4, LSHIFT, _) |
		(E4, RSHIFT, _) |
		(E5, PLUS, _) |
		(E5, MINUS, _) |
		(E6, PERCENT, _) |
		(E6, STAR, _) |
		(E6, SLASH, _) => return
			Operation::SHIFT(vec![LBR, Number(0), TILDA, Id("")]),

		(LBR, E1, _) => return
			Operation::SHIFT(vec![RBR, PIPE, CARET, AMPERSAND, LSHIFT, RSHIFT, PLUS, MINUS, PERCENT, STAR, SLASH]),

		(DataType(_), Vals, COMMA) => return
			Operation::SHIFT(vec![COMMA]),

		(DataType(_), Vals, _) => return
			Operation::REDUCE(2, &|vals| {
				if let (ExprKind::DType(size), ExprKind::Vals(arr)) =
					(vals[0].kind.clone(), vals[1].kind.clone()) {
					(Data, Expr{
						kind: ExprKind::Data(size, arr),
						span: vals[0].span.start..vals[1].span.end,
						..Default::default()
					})
				} else {unreachable!()}
			}),

		(TILDA, E7, _) => return
			Operation::REDUCE(2, &|vals| {
				(E7, Expr{
					kind: ExprKind::Not(Box::new(vals[1].clone())),
					span: vals[0].span.start..vals[1].span.end,
					..Default::default()
				})
			}),

		(IName(_), E, _) => return
			Operation::REDUCE(2, &|vals| {
				if let ExprKind::IName(op, size) = vals[0].kind {
					(Instr, Expr{
						kind: ExprKind::Instruction(op, size, vec![
							vals[1].clone()
						]),
						span: vals[0].span.start..vals[1].span.end,
						..Default::default()
					})
				} else {unreachable!()}}),

		_ => {}
	}}

	if stack.len() >= 1 { match (
		&stack[stack.len() - 1],
		&lookahead
	) {
		(IName(_), Reg(_)) |
		(IName(_), LBR) |
		(IName(_), Number(_)) |
		(IName(_), TILDA) |
		(IName(_), Id(_)) => return
			Operation::SHIFT(vec![Reg(""), LBR, Number(0), TILDA, Id("")]),

		(E1, PIPE) => return
			Operation::SHIFT(vec![PIPE]),

		(E2, CARET) => return
			Operation::SHIFT(vec![CARET]),

		(E3, AMPERSAND) => return
			Operation::SHIFT(vec![AMPERSAND]),

		(E4, LSHIFT) => return
			Operation::SHIFT(vec![LSHIFT]),

		(E4, RSHIFT) => return
			Operation::SHIFT(vec![RSHIFT]),

		(E5, PLUS) => return
			Operation::SHIFT(vec![PLUS]),

		(E5, MINUS) => return
			Operation::SHIFT(vec![MINUS]),

		(E6, PERCENT) => return
			Operation::SHIFT(vec![PERCENT]),

		(E6, STAR) => return
			Operation::SHIFT(vec![STAR]),

		(E6, SLASH) => return
			Operation::SHIFT(vec![SLASH]),

		(LBR, _) => return
			Operation::SHIFT(vec![LBR, Number(0), TILDA, Id("")]),

		(DataType(_), _) => return
			Operation::SHIFT(vec![LBR, Number(0), TILDA, Id("")]),

		(TILDA, _) => return
			Operation::SHIFT(vec![LBR, Number(0), TILDA, Id("")]),

		(E, _) => return
			Operation::REDUCE(1, &|vals| {
				(Vals, Expr{
					kind: ExprKind::Vals(vec![vals[0].clone()]),
					span: vals[0].span.clone(),
					..Default::default()
				})
			}),

		(Reg(_), _) => return
			Operation::REDUCE(1, &|vals| {
				(E, Expr{
					kind: vals[0].kind.clone(),
					span: vals[0].span.clone(),
					..Default::default()
				})
			}),

		(E1, _) => return
			Operation::REDUCE(1, &|vals| {
				(E, Expr{
					kind: vals[0].kind.clone(),
					span: vals[0].span.clone(),
					..Default::default()
				})
			}),

		(E2, _) => return
			Operation::REDUCE(1, &|vals| {
				(E1, Expr{
					kind: vals[0].kind.clone(),
					span: vals[0].span.clone(),
					..Default::default()
				})
			}),

		(E3, _) => return
			Operation::REDUCE(1, &|vals| {
				(E2, Expr{
					kind: vals[0].kind.clone(),
					span: vals[0].span.clone(),
					..Default::default()
				})
			}),

		(E4, _) => return
			Operation::REDUCE(1, &|vals| {
				(E3, Expr{
					kind: vals[0].kind.clone(),
					span: vals[0].span.clone(),
					..Default::default()
				})
			}),

		(E5, _) => return
			Operation::REDUCE(1, &|vals| {
				(E4, Expr{
					kind: vals[0].kind.clone(),
					span: vals[0].span.clone(),
					..Default::default()
				})
			}),

		(E6, _) => return
			Operation::REDUCE(1, &|vals| {
				(E5, Expr{
					kind: vals[0].kind.clone(),
					span: vals[0].span.clone(),
					..Default::default()
				})
			}),

		(E7, _) => return
			Operation::REDUCE(1, &|vals| {
				(E6, Expr{
					kind: vals[0].kind.clone(),
					span: vals[0].span.clone(),
					..Default::default()
				})
			}),

		(E8, _) => return
			Operation::REDUCE(1, &|vals| {
				(E7, Expr{
					kind: vals[0].kind.clone(),
					span: vals[0].span.clone(),
					..Default::default()
				})
			}),

		(Id(_), _) => return
			Operation::REDUCE(1, &|vals| {
				(E7, Expr{
					kind: vals[0].kind.clone(),
					span: vals[0].span.clone(),
					..Default::default()
				})
			}),

		(Number(_), _) => return
			Operation::REDUCE(1, &|vals| {
				(E7, Expr{
					kind: vals[0].kind.clone(),
					span: vals[0].span.clone(),
					..Default::default()
				})
			}),

		(IName(_), _) => return
			Operation::REDUCE(1, &|vals| {
				if let ExprKind::IName(op, size) = vals[0].kind {
					(Instr, Expr{
						kind: ExprKind::Instruction(op, size, vec![]),
						span: vals[0].span.clone(),
						..Default::default()
					})
				} else {unreachable!()}}),

		_ => {}
	}}

	match lookahead {
		EOI => Operation::NOMATCH,
		_ => Operation::SHIFT(vec![IName(""), DataType(""), Label("")]),
	}
}
