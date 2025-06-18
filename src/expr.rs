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
	Lsh(Box<Expr<'a>>, Box<Expr<'a>>),
	Rsh(Box<Expr<'a>>, Box<Expr<'a>>),
}

#[derive(Debug, Clone)]
pub struct Expr<'a> {
	pub kind: ExprKind<'a>,
	pub span: Span,
	pub size: u64,
	pub offset: u64,
}

enum InstrArgs {
	None,
	R1,
	R2,
	R3,
	Num8,
	Num64,
}


use InstrArgs::*;


const INSTRS: [[InstrArgs; 3]; 37] = [
	[R3,    R2,   Num64], // sto
	[R1,    R2,   Num64], // loa
	[R1,    R2,   R3],    // add
	[R1,    R2,   R3],    // sub
	[R1,    R2,   R3],    // mul
	[R1,    R2,   R3],    // div
	[R1,    R2,   Num64], // addn
	[R1,    R2,   Num64], // subn
	[R1,    R2,   Num64], // muln
	[R1,    R2,   Num64], // divn
	[R1,    R2,   Num64], // addz
	[R1,    R2,   Num64], // addc
	[R1,    R2,   Num64], // adds
	[R1,    R2,   R3],    // not
	[R1,    R2,   R3],    // and
	[R1,    R2,   R3],    // or
	[R1,    R2,   R3],    // xor
	[R1,    R2,   R3],    // shl
	[R1,    R2,   R3],    // shr
	[R1,    R2,   Num64], // andn
	[R1,    R2,   Num64], // orn
	[R1,    R2,   Num64], // xorn
	[R1,    R2,   Num64], // shln
	[R1,    R2,   Num64], // shrn
	[R3,    None, None],  // push
	[R1,    None, None],  // pop
	[R3,    None, None],  // call
	[Num8,  None, None],  // int
	[None,  None, None],  // iret
	[R2,    None, None],  // chst
	[R1,    None, None],  // lost
	[R2,    None, None],  // chtp
	[R1,    None, None],  // lotp
	[R2,    None, None],  // chflag
	[R1,    None, None],  // loflag
	[R1,    R3,   None],  // utok
	[R1,    R3,   None],  // ktou
];


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
			ExprKind::Xor(lhs, rhs) |
			ExprKind::Lsh(lhs, rhs) |
			ExprKind::Rsh(lhs, rhs) => {
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
				if *id == "$" {
					return self.offset as i64;
				}

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
			ExprKind::Sub(lhs, rhs) => lhs.eval(labels) - rhs.eval(labels),
			ExprKind::Mul(lhs, rhs) => lhs.eval(labels) * rhs.eval(labels),
			ExprKind::Div(lhs, rhs) => lhs.eval(labels) / rhs.eval(labels),
			ExprKind::Mod(lhs, rhs) => lhs.eval(labels) % rhs.eval(labels),
			ExprKind::And(lhs, rhs) => lhs.eval(labels) & rhs.eval(labels),
			ExprKind::Or(lhs, rhs) => lhs.eval(labels) | rhs.eval(labels),
			ExprKind::Xor(lhs, rhs) => lhs.eval(labels) ^ rhs.eval(labels),
			ExprKind::Not(c) => !c.eval(labels),
			ExprKind::Lsh(lhs, rhs) => lhs.eval(labels) << rhs.eval(labels),
			ExprKind::Rsh(lhs, rhs) => lhs.eval(labels) >> rhs.eval(labels),
			_ => unreachable!()
		}
	}

	pub fn to_bytes(&self, labels: &Vec<Expr>) -> Vec<u8> {
		match &self.kind {
			ExprKind::Label(_) => {vec![]},
			ExprKind::Instruction(op, size, args) => {
				let mut r1: u8 = 0;
				let mut r2: u8 = 0;
				let mut r3: u8 = 0;
				let mut num8: u8 = 0;
				let mut put_num64 = false;
				let mut num64: i64 = 0;

				for i in 0..3 {
					match INSTRS[*op as usize][i] {
						None => break,
						R1 => r1 = args[i].eval(labels) as u8,
						R2 => r2 = args[i].eval(labels) as u8,
						R3 => r3 = args[i].eval(labels) as u8,
						Num8 => num8 = args[i].eval(labels) as u8,
						Num64 => {
							put_num64 = true;
							num64 = args[i].eval(labels)
						},
					}
				}

				let mut result: Vec<u8> = vec![
					*op,
					r1 | (r2 << 4),
					r3 | (num8 & 0x0f << 4),
					(num8 & 0xf0 >> 4) | (size << 4),
				];

				if put_num64 {
					for i in 0..8 {
						result.push((num64 >> i * 8 & 0xff) as u8);
					}
				}

				result
			},
			ExprKind::Data(size, vals) => {
				let mut result: Vec<u8> = vec![];
				for i in vals {
					let val = i.eval(labels);
					for j in 0..*size {
						result.push((val >> j * 8 & 0xff) as u8);
					}
				}
				result
			},
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
