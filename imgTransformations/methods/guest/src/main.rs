#![no_main]

use risc0_zkvm::guest::env;
use std::io::Cursor;
use image::{load_from_memory, DynamicImage, ImageOutputFormat};

risc0_zkvm::guest::entry!(main);

fn apply_transformation(img: DynamicImage, name: &str) -> DynamicImage {
    match name {
        "grayscale" => img.grayscale(),
        "flipv" => img.flipv(),
        "fliph" => img.fliph(),
        _ => panic!("Unknown transformation: {}", name),
    }
}

fn main() {
    let flag: bool = env::read();

    let input_image_bytes: Vec<u8> = env::read(); 
    let transformations: Vec<String> = env::read();

    let mut img = load_from_memory(&input_image_bytes).expect("Failed to decode image");

    for t in &transformations {
        img = apply_transformation(img, t);
    }

    let mut buffer = Cursor::new(Vec::new());
    img.write_to(&mut buffer, ImageOutputFormat::Png).expect("Failed to encode image");
    let output_bytes = buffer.into_inner();

    if flag {
        env::commit(&(input_image_bytes, transformations, output_bytes));
    } else {
        env::commit(&(transformations,output_bytes,));
    }
}
