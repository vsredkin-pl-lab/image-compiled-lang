use image::{ImageBuffer, RgbImage, Rgb};
use crate::execution::chunk::{Chunk};


pub fn from_img(input: RgbImage) -> Option<Chunk> {
    let mut res = Chunk::new();
    todo!();
    Some(res)
}

pub fn into_img(input: Chunk) -> RgbImage {
    todo!();
    ImageBuffer::new(32,32)
}

