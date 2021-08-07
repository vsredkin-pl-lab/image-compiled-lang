use std::env;
use crate::text_parsing::parse_file;

mod text_parsing;
mod img_generation;

use std::path::PathBuf;
use image::{ImageBuffer, Rgb, RgbImage};
use num_integer::Roots;

/*
compiles lisp-like expressions into image
*/

fn main() {
    match env::args().skip(1).next() {
        Some(filename) => {
            let img = parse_file(&filename)
                .and_then(img_generation::compile)
                .expect("error");

            let mut result_filename = PathBuf::from(filename);
            result_filename.set_extension("bmp");

            let size = (img.len()/3).sqrt();
            println!("{}, {}", img.len(),size);

           let img:ImageBuffer<Rgb<u8>, Vec<_>> =
               ImageBuffer::from_vec(size as u32, size as u32, img).unwrap();

            img.save(result_filename).unwrap();



        }
        None => println!("usage: iclc <filename>")
    }
}