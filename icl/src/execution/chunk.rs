use std::ops::AddAssign;
#[derive(Debug)]
pub enum Opcode {
    Load(u8),
    LoadConst(u8),
    LoadImmediate(i8),
    Store(u8),
    Add, Sub, Mul, Div,
    Print
}
#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<Opcode>,
    pub constants: Vec<i64>
}

impl Chunk{
    pub fn new() -> Chunk{
        Chunk{code:vec![], constants:vec![]}
    }
}

impl AddAssign<Opcode> for Chunk{
    fn add_assign(&mut self, rhs: Opcode) {
        self.code.push(rhs)
    }
}