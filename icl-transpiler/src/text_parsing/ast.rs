#[derive(Debug)]
pub enum Stmt {
    Define(String),
    Assign(String, Box<Expr>),
    ExprStmt(Box<Expr>)
}

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Variable(String),
    Binary(Op, Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>)
}
#[derive(Debug)]
pub enum Op {
    Mul,
    Div,
    Add,
    Sub,
    Assign,
}

pub type Program = Vec<Stmt>;