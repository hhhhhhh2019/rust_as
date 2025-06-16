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
