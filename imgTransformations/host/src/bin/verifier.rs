use risc0_zkvm::Receipt;
use std::{env, fs};
use image::{load_from_memory, DynamicImage};

use zk_img_methods::ZK_IMG_METHOD_ID;

fn load_image_bytes(path: &str) -> Vec<u8> {
    fs::read(path).expect("Failed to read image file")
}

fn images_are_equal(img1: &DynamicImage, img2: &DynamicImage) -> bool {
    img1.to_rgba8() == img2.to_rgba8()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: verifier <receipt_file> <public_image>");
        std::process::exit(1);
    }

    let receipt_path = &args[1];
    let public_img_path = &args[2];

    let receipt_bytes = fs::read(receipt_path).expect("Failed to read receipt file");
    let receipt: Receipt = bincode::deserialize(&receipt_bytes).expect("Failed to deserialize receipt");
    receipt.verify(ZK_IMG_METHOD_ID).expect("Verification failed");
    println!("Receipt verified successfully.");

    let result: Result<(Vec<String>, Vec<u8>), _> = receipt.journal.decode();
    let (transformations, receipt_img_bytes) = result.expect("Failed to decode receipt journal");

    let receipt_img = load_from_memory(&receipt_img_bytes).expect("Failed to decode image from receipt");
    let public_img_bytes = load_image_bytes(public_img_path);
    let public_img = load_from_memory(&public_img_bytes).expect("Failed to decode public image");

    if images_are_equal(&receipt_img, &public_img) {
        println!("Image in receipt matches the public image.");
        println!("Transformation(s) applied: {}", transformations.join(" -> "));
    } else {
        println!("Image in receipt does NOT match the public image.");
    }
}
