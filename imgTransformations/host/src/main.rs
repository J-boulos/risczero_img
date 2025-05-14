use zk_img_methods::ZK_IMG_METHOD_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::{env, fs::File};
use std::io::Write;


fn load_image_bytes(path: &str) -> Vec<u8> {
    std::fs::read(path).expect("Failed to read image bytes")
}


fn run_single_proof(
    image_bytes: Vec<u8>,
    transformations: Vec<String>,
    recursive: bool,
    recursive_id: Option<u32>,
) {
    let mut builder = ExecutorEnv::builder();

    let env_builder = builder
        .write(&recursive).expect("Failed to write recursive flag")
        .write(&image_bytes).expect("Failed to write image bytes")
        .write(&transformations).expect("Failed to write transformations");

    let env = env_builder.build().expect("Failed to build zk env");

    let prover = default_prover();
    let prove_info = prover.prove(env, ZK_IMG_METHOD_ELF).expect("Proof generation failed");

    let receipt = prove_info.receipt;
    let receipt_bytes = bincode::serialize(&receipt).expect("Failed to serialize receipt");

    let (filename, image_filename) = match recursive_id {
        Some(id) => (
            format!("receipt_{}.bin", id),
            format!("image_{}.png", id),
        ),
        None => {
            let label = transformations.join("_");
            (
                format!("receipt_{}.bin", label),
                format!("img_{}.png", label),
            )
        }
    };

    if recursive {
        let (_input_image, _transforms, output_image): (Vec<u8>, Vec<String>, Vec<u8>) = 
            receipt.journal.decode().expect("Failed to decode recursive journal");
    
        File::create(&image_filename)
            .and_then(|mut f| f.write_all(&output_image))
            .expect("Failed to write output image");
    } else {
        let (_transforms, output_image): (Vec<String>, Vec<u8>) =
            receipt.journal.decode().expect("Failed to decode non-recursive journal");
    
        File::create(&image_filename)
            .and_then(|mut f| f.write_all(&output_image))
            .expect("Failed to write output image");
    }

    File::create(&filename)
        .and_then(|mut f| f.write_all(&receipt_bytes))
        .expect("Failed to save receipt");

    println!("Saved receipt to: {}", filename);
    
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage:");
        eprintln!("{} <image_path> <transformation1> [transformation2 ...] [--recursive <id>]", args[0]);
        std::process::exit(1);
    }

    let image = load_image_bytes(&args[1]);

    let mut recursive = false;
    let mut recursive_id = None;
    let mut transformations: Vec<String> = Vec::new();

    let mut i = 2;
    while i < args.len() {
        if args[i] == "--recursive" {
            recursive = true;
            if i + 1 < args.len() {
                if let Ok(id) = args[i + 1].parse::<u32>() {
                    recursive_id = Some(id);
                    i += 1; 
                }
            }
        } else {
            transformations.push(args[i].clone());
        }
        i += 1;
    }

    run_single_proof(image, transformations, recursive, recursive_id);

}
