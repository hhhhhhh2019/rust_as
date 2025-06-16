pub fn opcode(s: &str) -> u8 {
	match s {
		"sto"    => 0x0,
		"loa"    => 0x1,
		"add"    => 0x2,
		"sub"    => 0x3,
		"mul"    => 0x4,
		"idiv"   => 0x5,
		"addn"   => 0x6,
		"subn"   => 0x7,
		"muln"   => 0x8,
		"divn"   => 0x9,
		"addz"   => 0xa,
		"addc"   => 0xb,
		"adds"   => 0xc,
		"notr"   => 0xd,
		"andr"   => 0xe,
		"orr"    => 0xf,
		"xorr"   => 0x10,
		"shl"    => 0x11,
		"shr"    => 0x12,
		"andn"   => 0x13,
		"orn"    => 0x14,
		"xorn"   => 0x15,
		"shln"   => 0x16,
		"shrn"   => 0x17,
		"push"   => 0x18,
		"pop"    => 0x19,
		"call"   => 0x1a,
		"iint"   => 0x1b,
		"iret"   => 0x1c,
		"chst"   => 0x1d,
		"lost"   => 0x1e,
		"chtp"   => 0x1f,
		"lotp"   => 0x20,
		"chflag" => 0x21,
		"loflag" => 0x22,
		"utok"   => 0x23,
		"ktou"   => 0x24,
		_ => unreachable!()
	}
}


pub fn register(s: &str) -> u8 {
	match s {
		"r0"   => 0,
		"r1"   => 1,
		"r2"   => 2,
		"r3"   => 3,
		"r4"   => 4,
		"r5"   => 5,
		"r6"   => 6,
		"r7"   => 7,
		"r8"   => 8,
		"r9"   => 9,
		"r10"  => 10,
		"r11"  => 11,
		"r12"  => 12,
		"r13"  => 13,
		"r14"  => 14,
		"r15"  => 15,
		"pc"   => 15,
		"sp"   => 14,
		_      => unreachable!(),
	}
}


pub fn datatype(s: &str) -> u8 {
	match s {
		"db" => 1,
		"ds" => 2,
		"di" => 4,
		"dl" => 8,
		_    => unreachable!(),
	}
}
