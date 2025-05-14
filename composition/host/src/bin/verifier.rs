use std::{env, fs};
use risc0_zkvm::Receipt;
use methods::COMPOSITION_ID;

use image::{RgbImage,load_from_memory,DynamicImage};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args.len() > 4 {
        eprintln!("Usage: verifier <receipt_file> <public_image>");
        std::process::exit(1);
    }

    let receipt_path = &args[1];
    let expected_image_path = &args[2];
    let parallel_mode = args.contains(&"--parallel".to_string());


    let receipt_bytes = fs::read(receipt_path).expect("Failed to read receipt file");
    let receipt: Receipt = bincode::deserialize(&receipt_bytes).expect("Failed to deserialize receipt");
    receipt.verify(COMPOSITION_ID).expect("Receipt verification failed");
    
    if parallel_mode {

    let (raw_rgb, width, height, _transformations): (Vec<u8>, u32, u32, Vec<String>) =
        receipt.journal.decode().expect("Failed to decode journal");

    let image = RgbImage::from_raw(width, height, raw_rgb.clone())
        .expect("Failed to construct image from raw RGB data");

    /*let output_path = "receipt_output.png";
    image.save_with_format(output_path, ImageFormat::Png)
        .expect("Failed to save receipt image");
    println!("Receipt image saved to: {}", output_path);*/

    let expected_img = image::open(expected_image_path)
        .expect("Failed to open expected image")
        .to_rgb8();

    let expected_bytes = expected_img.into_raw();
    let actual_bytes = image.into_raw();

    if actual_bytes == expected_bytes {
        println!("Image matches receipt output!");
    } else {
        eprintln!("Image does NOT match the output in the receipt.");
        std::process::exit(1);
    }
}
else 
{
    let result: Result<( Vec<u8> , Vec<String>), _> = receipt.journal.decode();
        let (receipt_img_bytes,transformations) = result.expect("Failed to decode receipt journal");

        let receipt_img = load_from_memory(&receipt_img_bytes).expect("Failed to decode image from receipt");
        let public_img_bytes = load_image_bytes(expected_image_path);
        let public_img = load_from_memory(&public_img_bytes).expect("Failed to decode public image");

        if images_are_equal(&receipt_img, &public_img) {
            println!("Image in receipt matches the public image.");
            println!("Transformation(s) applied: {}", transformations.join(" -> "));
        } else {
            println!("Image in receipt does NOT match the public image.");
        }
    }

}

fn load_image_bytes(path: &str) -> Vec<u8> {
    fs::read(path).expect("Failed to read public image file")
}

fn images_are_equal(img1: &DynamicImage, img2: &DynamicImage) -> bool {
    img1.to_rgba8() == img2.to_rgba8()
}


