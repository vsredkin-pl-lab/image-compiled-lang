use image::{RgbImage, Rgb, ImageBuffer};
use crate::text_parsing::ast;
use std::collections::{HashMap, LinkedList};
use crate::text_parsing::ast::{Expr, Stmt, Program, Op};

use IntoIterator;

use rand::Rng;
use std::iter;
use num_integer::Roots;


pub fn compile(input: ast::Program) -> Result<Vec<u8>, String> {
    let variables = collect_variable_names(&input)?;
    let mut compiler = Compiler{
        variables,
        code: ColorList::new()
    };
    let mut vec = compiler.compile(&input)?;

    let size = vec.len();

    dbg!(size);

    let target_size = ((size/3).sqrt()+1).pow(2)*3;
    dbg!(target_size);
    let extension = iter::repeat(255).take(target_size-size);
    vec.extend(extension);
    Ok(vec)
}

type ColorMap = HashMap<String, (u8, u8, u8)>;
type ColorList = Vec<u8>;

fn collect_variable_names(input: &ast::Program) -> Result<ColorMap, String> {
    let mut res = HashMap::new();
    for stmt in input {
        visit_collect_varname(stmt, &mut res)?;
    }
    Ok(res)
}

fn visit_collect_varname(input: &ast::Stmt, res: &mut ColorMap) -> Result<(), String>{
    match input {
        Stmt::Define(varname) => {
            if res.contains_key(varname) {
                Err(format!("redefinition of {}", varname))?;
            }else {
                //generate random color for variable
                let mut rng = rand::thread_rng();
                let mut triplet:(u8,u8,u8);

                'lop: loop {
                    triplet = (rng.gen(), rng.gen(), rng.gen());
                    for (k, v) in &*res {
                        if v==&triplet { continue 'lop;}
                    }
                    break;
                }

                res.insert(varname.clone(), triplet);
            }
        }
        _ => {}
    }
    Ok(())
}

struct Compiler {
    variables: ColorMap,
    code: ColorList
}

trait TagPushable {
    fn push_tag_random(&mut self, tag:u8);
}

impl TagPushable for ColorList {
    fn push_tag_random(&mut self, tag: u8) {
        let mut rng = rand::thread_rng();
        self.extend([tag, rng.gen(), rng.gen()]);
    }
}

impl Compiler {
    fn new() -> Compiler {
        Compiler{code:ColorList::new(), variables:HashMap::new()}
    }



    fn compile(&mut self, input: &Program) -> Result<Vec<u8>, String> {
        for stmt in input{
            self.visit_stmt(stmt)?;
        }
        Ok(self.code.clone())
    }

    fn visit_stmt(&mut self, input: &Stmt) -> Result<(), String> {
        match input {
            Stmt::Define(varname) => {
                self.code.push_tag_random(0); // (
                self.code.push_tag_random(10); //define

                let triplet = self.variables.get(varname)
                    .ok_or(format!("unknown variable {}", varname))?;
                self.code.extend([triplet.0, triplet.1, triplet.2]);

                self.code.push_tag_random(1); //)
            }
            Stmt::Assign(target, expr) => {
                self.code.push_tag_random(0);// (
                self.code.push_tag_random(9); //assign

                let triplet = self.variables.get(target)
                    .ok_or(format!("undefined variable {}", target))?;

                self.code.extend([triplet.0, triplet.1, triplet.2]);
                self.visit_expr(expr)?;
                self.code.push_tag_random(1);
            }
            Stmt::ExprStmt(expr) => {
                self.visit_expr(expr)?;
            }
        }
        Ok(())
    }

    fn visit_expr(&mut self, input: &Expr) -> Result<(), String> {
        let mut rng = rand::thread_rng();

        match input {
            Expr::Number(n) => {
                self.code.push_tag_random(8);


                //let pairs = n.to_be_bytes().iter().collect::<Vec<_>>().chunks(2);

                let num_vec = n.to_be_bytes().iter().map(|t| *t).collect::<Vec<u8>>();
                let pairs = num_vec.chunks(2);

                for chunk in  pairs.into_iter() {
                    let n1 = chunk[0];
                    let n2 = chunk[1];
                    let n3 = rng.gen();

                    self.code.extend([n1,n2,n3]);
                }

            }
            Expr::Variable(varname) => {
                self.code.push_tag_random(6);
                let triplet = self.variables
                    .get(varname)
                    .ok_or(format!("undefined variable {}", varname))?;
                self.code.extend([triplet.0, triplet.1, triplet.2]);
            }

            Expr::Binary(op, left, right) => {
                self.code.push_tag_random(0); // (

                let tag = match op {
                    Op::Mul => 4,
                    Op::Div => 5,
                    Op::Add => 2,
                    Op::Sub => 3,
                    Op::Assign => {
                        panic!("assign in expression is not supported");
                    }
                };
                self.code.push_tag_random(tag);

                self.visit_expr(left)?;
                self.visit_expr(right)?;

                self.code.push_tag_random(1) // )
            }
            Expr::Call(name, args) => {
                self.code.push_tag_random(0); // (
                match name.as_str() {
                    "print" => {
                        self.code.push_tag_random(7);
                        for arg in args {
                            self.visit_expr(arg)?;
                        }
                    }
                    x => {
                        Err(format!("unsupported function {}", x))?;
                    }
                }
                self.code.push_tag_random(1); // )
            }
        }
        Ok(())
    }
}