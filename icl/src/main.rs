use std::env;
use crate::execution::{chunk_compiler, vm};

mod execution;
mod image_parsing;

fn main() {
    match env::args().skip(1).next() {
        Some(filename) => {
            let tree = image_parsing::parse_image_file(&filename)
                .expect("error");
            println!("{:?}", tree);

            let chunk = chunk_compiler::compile(&tree)
                .expect("error");

            println!("{:?}", chunk);

            vm::VM::run_chunk(&chunk)
                .expect("error");

        }
        None => println!("usage: icl <filename>")
    }
}
