use crate::types::RGBA;

pub type VarType = (Option<String>, Option<String>);
pub type VarName = String;

#[derive(Debug)]
pub struct Var(pub VarType, pub VarName);

#[derive(Debug)]
pub struct VarParam(pub Var, pub Option<Box<Expr>>);

#[derive(Debug)]
pub enum Statement {
    Import(String),
    UnpackTuple(Vec<VarName>, Box<Expr>),
    ConstDef(Var, Box<Expr>),
    SeriesDef(Var, Box<Expr>),
    TypeDef(String, Vec<(String, String)>),
    EnumDef(String, Vec<(String, Option<String>)>),
    VarIpDef(Var, Box<Expr>),
    VarDef(Var, Box<Expr>),
    VarLet(Var, Box<Expr>),
    VarAssign(VarName, Box<Expr>),
    ForTo(Var, Box<Expr>, Box<Expr>, Vec<Statement>, Option<Box<Expr>>),
    ForIn(Var, Box<Expr>, Vec<Statement>),
    While(Box<Expr>, Vec<Statement>),
    FnDef(String, Vec<VarParam>, Vec<Statement>),
    Expression(Box<Expr>),
}

pub type CallArguments = Vec<(Option<VarName>, Box<Expr>)>;

#[derive(Debug)]
pub enum Expr {
    Identifier(String),
    String(String),
    Bool(bool),
    Int(i64),
    Float(f64),
    MakeTuple(Vec<Box<Expr>>),
    Op(Box<Expr>, Opcode, Box<Expr>), 
    If(Box<Expr>, Vec<Statement>, Option<Vec<Statement>>),
    Index(String, Box<Expr>),
    Switch(Option<Box<Expr>>, Vec<(Option<Box<Expr>>, Box<Statement>)>),
    Not(Box<Expr>),
    Negative(Box<Expr>),
    HashColor(RGBA),
    FnCall(String, Option<Vec<String>>, CallArguments),
    MethodCall(VarName, String, Option<String>, CallArguments),
    PropertyAccess(VarName, String)
}

#[derive(Debug)]
pub enum Opcode {
    TernaryIf,
    TernaryElse,
    NotEqual,
    Equal,
    Greater,
    Gte,
    Less,
    Lte,
    And,
    Or,
    Mul,
    Div,
    Add,
    Sub,
    Mod
}

#[derive(Debug)]
pub struct Program {
    version: i32,
    exprs: Vec<Box<Expr>>
}
