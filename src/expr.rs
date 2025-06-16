use logos::Span;


#[derive(Debug, Clone)]
pub enum ExprKind<'a> {
	None,

	Number(i64),
	Reg(u8),
	Id(&'a str),
	Label(&'a str),
	IName(u8, u8),
	DType(u8),

	Instruction(u8, u8, Vec<Expr<'a>>),
	Data(u8, Vec<Expr<'a>>),

	Vals(Vec<Expr<'a>>),

	Sum(Box<Expr<'a>>, Box<Expr<'a>>),
	Sub(Box<Expr<'a>>, Box<Expr<'a>>),
	Mul(Box<Expr<'a>>, Box<Expr<'a>>),
	Div(Box<Expr<'a>>, Box<Expr<'a>>),
	Mod(Box<Expr<'a>>, Box<Expr<'a>>),
	And(Box<Expr<'a>>, Box<Expr<'a>>),
	Or(Box<Expr<'a>>, Box<Expr<'a>>),
	Xor(Box<Expr<'a>>, Box<Expr<'a>>),
	Not(Box<Expr<'a>>),
}

#[derive(Debug, Clone)]
pub struct Expr<'a> {
	pub kind: ExprKind<'a>,
	pub span: Span,
	pub size: u64,
	pub offset: u64,
}


impl Expr<'_> {
	pub fn update_offset(&mut self, offset: u64) {
		self.offset = offset;

		match &mut self.kind {
			ExprKind::Instruction(_, _, arr) => {
				for i in arr {
					i.update_offset(offset);
				}
			},
			ExprKind::Data(_, arr) => {
				for i in arr {
					i.update_offset(offset);
				}
			},
			ExprKind::Sum(lhs, rhs) |
			ExprKind::Sub(lhs, rhs) |
			ExprKind::Mul(lhs, rhs) |
			ExprKind::Div(lhs, rhs) |
			ExprKind::Mod(lhs, rhs) |
			ExprKind::And(lhs, rhs) |
			ExprKind::Or(lhs, rhs) |
			ExprKind::Xor(lhs, rhs) => {
				lhs.update_offset(offset);
				rhs.update_offset(offset);
			},

			ExprKind::Not(c) => {
				c.update_offset(offset);
			},
			_ => {}
		}
	}

	fn eval(&self, labels: &Vec<Expr>) -> i64 {
		match &self.kind {
			ExprKind::Number(n) => *n,
			ExprKind::Id(id) => {
				for l in labels {
					if let ExprKind::Label(name) = l.kind {
						if name == *id {
							return l.offset as i64;
						}
					} else {unreachable!()}
				}

				println!("label {id} not found");
				unimplemented!()
			},
			ExprKind::Reg(n) => *n as i64,
			ExprKind::Sum(lhs, rhs) => lhs.eval(labels) + rhs.eval(labels),
			ExprKind::Sub(lhs, rhs) => lhs.eval(labels) + rhs.eval(labels),
			ExprKind::Mul(lhs, rhs) => lhs.eval(labels) + rhs.eval(labels),
			ExprKind::Div(lhs, rhs) => lhs.eval(labels) + rhs.eval(labels),
			ExprKind::Mod(lhs, rhs) => lhs.eval(labels) + rhs.eval(labels),
			ExprKind::And(lhs, rhs) => lhs.eval(labels) + rhs.eval(labels),
			ExprKind::Or(lhs, rhs) => lhs.eval(labels) + rhs.eval(labels),
			ExprKind::Xor(lhs, rhs) => lhs.eval(labels) + rhs.eval(labels),
			ExprKind::Not(c) => !c.eval(labels),
			_ => unreachable!()
		}
	}

	pub fn to_bytes(&self, labels: &Vec<Expr>) -> Vec<u8> {
		match self.kind {
			// ExprKind::Data(size, vals) {

			// },
			_ => unreachable!()
		}
	}
}


impl Default for Expr<'_> {
	fn default() -> Self {
		Expr {
			kind: ExprKind::None,
			span: 0..0,
			size: 0,
			offset: 0,
		}
	}
}
