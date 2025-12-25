pub struct Module {
	pub path: String,
	pub imports: Vec<(String, Vec<String>)>,
	pub types: Vec<Type>,
	pub symbols: Vec<Symbol>,
}

pub struct Symbol {
	pub name: String,
	pub typ_: Type,
	pub expr: Option<Expr>,
}

pub struct Type {
	pub name: Option<String>,
	pub generics: Option<Vec<Type>>,
	pub def: TypeDef,
}

pub enum TypeDef {
	Alias { alias: Box<Type> },
	Fn { args: Vec<Symbol>, vararg: Option<Box<Symbol>>, rets: Option<Box<Type>> },
	Pointer { pointee: Box<Type> },
	Struct { extends: Option<Box<Type>>, fields: Vec<Symbol> },
	Union { extends: Option<Box<Type>>, fields: Vec<Symbol> },
	Enum { extends: Option<Box<Type>>, fields: Vec<Symbol> },
	U8, U16, U32, U64, U128, Int,
	I8, I16, I32, I64, I128, Uint,
	F16, F32, F64, F128, Float,
}

pub enum Expr {
	TypeConv(Box<Expr>, Box<Type>),
	Comptime(Box<Expr>),
	Symbol(String),
	Int(isize),
	Float(isize, isize),
	UnOp(UnOp, Box<Expr>),
	BinOp(Box<Expr>, BinOp, Box<Expr>),
	Index(Box<Expr>, Box<Expr>),
	Field(Box<Expr>, String),
	Call(Box<Expr>, Vec<Expr>, Option<Box<Expr>>),
	Lambda(Vec<(Option<Type>, String)>, Option<Type>, Vec<Instruction>),
	Struct(Option<Box<Type>>, Vec<(Option<String>, Expr)>),
	Array(Vec<Expr>),
	Deref(Box<Symbol>),
	Ref(Box<Symbol>),
}

pub enum UnOp {
	Not, BitNot,
	PreIncrement,
	PostIncrement,
	PreDecrement,
	PostDecrement,
}

pub enum BinOp {
	Assign(Box<BinOp>, Box<Expr>),
	Add, Sub, Mul, Div, Mod,
	And, Or, Xor, BitAnd, BitOr, BitXor,
	Eq, Neq, Less, LessEq, Greater, GreaterEq,
	ShiftL, ShiftR,
}

pub enum Instruction {
	Comptime(Box<Instruction>),
	
}
