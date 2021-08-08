use crate::image_parsing::ast::Program;

use image::io::Reader;

pub mod ast;
pub mod token;

#[allow(unused_imports)]
pub mod image_tree;


pub fn parse_image_file(filename: &str) -> Result<Program, String> {

    let image = Reader::open(filename)
        .or(Err("failed to open file"))?
        .decode()
        .map_err(|e| {e.to_string()})?;

    let buffer = image.as_rgb8().ok_or("wrong image format")?;
    let buffer = buffer.as_raw();

    let parser = image_tree::ProgramParser::new();
    let tokens = token::tokenize(buffer);
    parser.parse(tokens)
        .or(Err("error parsing image grammar".to_string()))
}