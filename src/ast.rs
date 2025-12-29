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
	Extends { base: Box<Type>, def: Option<Box<TypeDef>>, },
	Sized { size: usize, },
	Fn { args: Vec<Symbol>, vararg: Option<Box<Symbol>>, rets: Option<Box<Type>>, },
	Pointer { pointee: Box<Type>, },
	Struct { fields: Vec<Symbol>, },
	Union { fields: Vec<Symbol>, },
	Enum { fields: Vec<Symbol>, },
}

pub enum Expr {
	Comptime(Box<Expr>),
	Symbol(String),
	Int(isize),
	Float(isize, usize),
	UnOp(UnOp, Box<Expr>),
	BinOp(Box<Expr>, BinOp, Box<Expr>),
	TypeConv(Box<Expr>, Box<Type>),
	Index(Box<Expr>, Box<Expr>),
	Field(Box<Expr>, String),
	Call(Box<Expr>, Vec<Expr>, Option<Box<Expr>>),
	FnBody(Vec<(Option<Type>, String)>, Option<Type>, Vec<Instruction>),
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
	Add, Sub, Mul, Div, Mod,
	And, Or, Xor, BitAnd, BitOr, BitXor,
	Eq, Neq, Less, LessEq, Greater, GreaterEq,
	ShiftL, ShiftR,
}

pub enum Instruction {
	Comptime(Box<Instruction>),
	Next,
	Break,
	Return(Option<Expr>),
	VarDecl(Type, String),
	Assign(Vec<String>, Expr),
	If(Expr, Vec<Instruction>),
	ElseIf(Expr, Vec<Instruction>),
	Else(Vec<Instruction>),
	While(Expr, Vec<Instruction>),
}


