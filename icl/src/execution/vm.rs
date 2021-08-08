use crate::execution::chunk::{Chunk, Opcode};

pub(crate) struct VM{
    stack: Vec<i64>
}

impl VM {

    fn new() -> Self {
        VM{stack: vec![]}
    }

    pub fn run_chunk(chunk: &Chunk) -> Result<(), String> {
        let mut vm = VM::new();
        for instruction in &chunk.code {
            match instruction {
                Opcode::Load(idx) => {
                    let item = *vm.stack.get((*idx) as usize)
                        .ok_or(format!("error loading stack element {}", idx))?;
                    vm.stack.push(item);
                }

                Opcode::LoadImmediate(n) => {
                    let item = *n as i64;
                    vm.stack.push(item);
                }

                Opcode::LoadConst(idx) => {
                    let item = *chunk.constants.get((*idx) as usize)
                        .ok_or("error loading constant: index out of bounds")?;
                    vm.stack.push(item);
                }

                Opcode::Store(idx) => {
                    let item = vm.stack.pop()
                        .ok_or("stack underflow")?;
                    if vm.stack.len()<=(*idx) as usize {
                        Err("error storing item: index out of stack bounds")?;
                    }
                    vm.stack[*idx as usize] = item;
                }

                Opcode::Add => {
                    let item2 = vm.stack.pop().ok_or("stack underflow")?;
                    let item1 = vm.stack.pop().ok_or("stack underflow")?;
                    let res = item1 + item2;
                    vm.stack.push(res);
                }

                Opcode::Sub => {
                    let item2 = vm.stack.pop().ok_or("stack underflow")?;
                    let item1 = vm.stack.pop().ok_or("stack underflow")?;
                    let res = item1 - item2;
                    vm.stack.push(res);
                }
                Opcode::Mul => {
                    let item2 = vm.stack.pop().ok_or("stack underflow")?;
                    let item1 = vm.stack.pop().ok_or("stack underflow")?;
                    let res = item1 * item2;
                    vm.stack.push(res);
                }
                Opcode::Div => {
                    let item2 = vm.stack.pop().ok_or("stack underflow")?;
                    let item1 = vm.stack.pop().ok_or("stack underflow")?;

                    if item2==0 {
                        Err("zero division")?;
                    }

                    let res = item1 / item2;
                    vm.stack.push(res);
                }

                Opcode::Print => {
                    let item = vm.stack.pop().ok_or("stack underflow")?;
                    println!("{}", item);
                }
            }
        }
        Ok(())
    }
}