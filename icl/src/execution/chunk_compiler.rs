use crate::execution::chunk::{Chunk, Opcode};
use crate::image_parsing::ast::{Program, Stmt, Expr, Op};
use std::collections::HashMap;

pub fn compile(program:&Program) -> Result<Chunk, String> {
    let mut res = Chunk::new();
    let variable_map = assign_variables(program)?;

    let mut compiler = Compiler{variable_map, constant_map:HashMap::new()};

    for stmt in program{
        match stmt{
            Stmt::Define(_x) => {
                res += Opcode::LoadImmediate(0);
            }
            Stmt::Assign(target, expr) => {
                compiler.visit_compile_expr(expr, &mut res)?;

                let slot = *compiler.variable_map.get(target)
                    .ok_or(format!("unknown variable {}", decode_variable(*target)))?;
                res += Opcode::Store(slot);
            }
            Stmt::PrintStmt(expr) => {
                compiler.visit_compile_expr(expr, &mut res)?;
                res += Opcode::Print;
            }
        }
    }
    Ok(res)
}

fn assign_variables(program:&Program) -> Result<HashMap<usize, u8>, String> {
    let mut res = HashMap::new();
    for stmt in program{
        match stmt {
            Stmt::Define(variable) => {
                if res.contains_key(variable) {
                    Err(format!(
                        "redefinition of variable {}",
                        decode_variable(*variable)
                    ))?;
                }

                if res.len() == u8::MAX as usize {
                    Err("too many variables, compiler supports maximum of 255")?;
                }

                res.insert(*variable, res.len() as u8);
            }
            _ => {}
        }
    }
    Ok(res)
}

fn decode_variable(mut n:usize) -> String {
    //read 3 bytes of rgb into (r,g,b) string
    let b = n & 0xff;
    n>>=8;
    let g = n & 0xff;
    n>>=8;
    let r = n & 0xff;
    format!("({},{},{})", r, g, b)
}
struct Compiler {
    variable_map: HashMap<usize, u8>,
    constant_map: HashMap<i64, u8>,
}

impl Compiler{

    fn visit_compile_expr(&mut self, expr:&Expr, chunk: &mut Chunk) -> Result<(), String> {
        match expr {
            Expr::Number(x) => {
                let x = *x;
                if x>= i8::MIN as i64 && x<= i8::MAX as i64 {
                    *chunk += Opcode::LoadImmediate(x as i8);
                }else{

                    if self.constant_map.contains_key(&x){
                        *chunk += Opcode::LoadConst(*self.constant_map.get(&x).unwrap());
                        return Ok(());
                    }

                    if chunk.constants.len()== u8::MAX as usize {
                        Err("too many constants, compiler supports maximum of 255 long constants")?;
                    }

                    let idx = chunk.constants.len();
                    chunk.constants.push(x);

                    self.constant_map.insert(x, idx as u8);

                    *chunk += Opcode::LoadConst(x as u8);

                }
            }
            Expr::Variable(v) => {
                let stack_idx = *self.variable_map.get(v)
                    .ok_or(format!("unknown variable {}", decode_variable(*v)))?;
                *chunk += Opcode::Load(stack_idx);

            }
            Expr::Binary(op, left, right) => {
                self.visit_compile_expr(left, chunk)?;
                self.visit_compile_expr(right, chunk)?;

                *chunk += match op {
                    Op::Mul => {Opcode::Mul}
                    Op::Div => {Opcode::Div}
                    Op::Add => {Opcode::Add}
                    Op::Sub => {Opcode::Sub}
                };
            }
        }

        Ok(())
    }
}

