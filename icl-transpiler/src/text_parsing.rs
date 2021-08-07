use std::fs;

pub mod ast;
pub mod text_tree;

pub fn parse_file(filename: &str) -> Result<ast::Program, String> {
    let text = fs::read_to_string(filename).expect("failed to read file");
    let parser = text_tree::ProgramParser::new();

    dbg!(parser.parse(&text)
        .map_err(|e| e.to_string()))
}