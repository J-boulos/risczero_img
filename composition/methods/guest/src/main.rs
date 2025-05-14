use risc0_zkvm::guest::env;
use risc0_zkvm::serde;
use zk_img_methods::ZK_IMG_METHOD_ID;

use std::collections::HashSet;
use image::RgbImage;



risc0_zkvm::guest::entry!(main);


fn main() {

    let is_restitch: bool = env::read();

    if !is_restitch {

    let (mut transformations, mut image_bytes) : (Vec<String>, Vec<u8>) = env::read();
    
    let proof_input1 = serde::to_vec(&(transformations.clone(),image_bytes.clone(),)).unwrap();
    env::verify(ZK_IMG_METHOD_ID, &proof_input1).unwrap();


    let num_recursive: u32 = env::read();

    for _ in 0..num_recursive {
        let (prev_image_bytes, transformation, next_image_bytes): (Vec<u8>, Vec<String>, Vec<u8>) = env::read();
        
        let proof_input = serde::to_vec(&(prev_image_bytes.clone(), transformation.clone(), next_image_bytes.clone())).unwrap();
        env::verify(ZK_IMG_METHOD_ID, &proof_input).unwrap();

        if image_bytes != prev_image_bytes {
            panic!("Image bytes do not match: previous output != next input.");
        }

        image_bytes = next_image_bytes;
        for transformation in transformation {
            transformations.push(transformation);
        }
    }

    env::commit(&(image_bytes, transformations));
    
    }
    
    else {
        
        let num_parts: u32 = env::read();
        let mut all_transformations: HashSet<String> = HashSet::new();
        let mut decoded_images: Vec<RgbImage> = Vec::new();

        for _ in 0..num_parts {
            let (_input_bytes, _transformations, output_bytes): (Vec<u8>, Vec<String>, Vec<u8>) = env::read();

            let proof_input = serde::to_vec(&(_input_bytes.clone(), _transformations.clone(), output_bytes.clone())).unwrap();
            env::verify(ZK_IMG_METHOD_ID, &proof_input).unwrap();

            
            for t in _transformations {
                all_transformations.insert(t);
            }

            let image = image::load_from_memory(&output_bytes)
                .expect("Failed to decode PNG from output_bytes")
                .to_rgb8();

            decoded_images.push(image);
        }

        let width = decoded_images[0].width();
        let total_height: u32 = decoded_images.iter().map(|img| img.height()).sum();
        let mut stitched = RgbImage::new(width, total_height);

        let mut current_y = 0;
        for img in decoded_images {
            for y in 0..img.height() {
                for x in 0..img.width() {
                    let pixel = img.get_pixel(x, y);
                    stitched.put_pixel(x, current_y + y, *pixel);
                }
            }
            current_y += img.height();
        }
        let raw_rgb = stitched.into_raw();
        let transformations: Vec<String> = all_transformations.into_iter().collect();

        env::commit(&(raw_rgb, width, total_height, transformations));
    }
}


