use logos::Logos;


#[derive(Logos, Debug, Clone, Eq, PartialEq)]
#[logos(skip r"\s+")]
pub enum Token<'a> {
	#[token("+")]
	PLUS,
	#[token("-")]
	MINUS,
	#[token("*")]
	STAR,
	#[token("/")]
	SLASH,
	#[token("%")]
	PERCENT,
	#[token("&")]
	AMPERSAND,
	#[token("|")]
	PIPE,
	#[token("^")]
	CARET,
	#[token("~")]
	TILDA,
	#[token("(")]
	LBR,
	#[token(")")]
	RBR,
	#[token(",")]
	COMMA,

	#[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
	#[regex(r"0x[0-9a-fA-F]+", |lex| i64::from_str_radix(lex.slice(), 16).unwrap())]
	#[regex(r"0b[0-1]+", |lex| i64::from_str_radix(lex.slice(), 2).unwrap())]
	Number(i64),

	#[regex(r"(r[0-9]|r0[1-5]|sp|pc)")]
	Reg(&'a str),

	#[regex(r"(sto|loa|add|sub|mul|idiv|addn|subn|muln|divn|addz|addc|adds|notr|andr|orr|xorr|shl|shr|andn|orn|xorn|shln|shrn|push|pop|call|iint|iret|chst|lost|chtp|lotp|chflag|loflag|utok|ktou)[bsil]?")]
	IName(&'a str),

	#[regex(r"(db|ds|di|dl)")]
	DataType(&'a str),

	#[regex(r"[^0-9\s\+\-\*\/\%\(\)\|\^\&\~\,][^\s\+\-\*\/\%\(\)\|\^\&\~\,]*", |lex| lex.slice())]
	Id(&'a str),

	#[regex(r"[^0-9\s\+\-\*\/\%\(\)\|\^\&\~\,][^\s\+\-\*\/\%\(\)\|\^\&\~\,]*:", |lex| lex.slice().strip_suffix(":"))]
	Label(&'a str),

	EOI,

	Vals,
	Data,
	E,
	E1,
	E2,
	E3,
	E4,
	E5,
	E6,
	E7,
	Instr,
}
