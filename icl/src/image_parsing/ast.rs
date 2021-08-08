/*
same structures as in transpiler grammar, but identifiers are stored as (u8,u8,u8) colors,
transformed into usize for simplicity
*/

#[derive(Debug)]
pub enum Stmt {
    Define(usize),
    Assign(usize, Box<Expr>),
    PrintStmt(Box<Expr>)
}

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Variable(usize),
    Binary(Op, Box<Expr>, Box<Expr>)
}
#[derive(Debug)]
pub enum Op {
    Mul,
    Div,
    Add,
    Sub,
}

pub type Program = Vec<Stmt>;