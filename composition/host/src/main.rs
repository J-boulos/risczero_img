use methods::{COMPOSITION_ELF, COMPOSITION_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use std::{env, fs, path::Path};
use regex::Regex;

fn main() {

    let args: Vec<String> = env::args().collect();
    let is_parallel = args.contains(&"--parallel".to_string());

    let receipt_dir = Path::new("receipts");

    let mut initial_receipt_path: Option<_> = None;
    let mut recursive_receipt_paths: Vec<_> = Vec::new();

    let re_recursive = Regex::new(r"_\d+\.bin$").unwrap();

    for entry in fs::read_dir(receipt_dir).expect("Failed to read receipts directory") {
        let path = entry.expect("Invalid dir entry").path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("bin") {
            let filename = path.file_name().unwrap().to_string_lossy();
            if re_recursive.is_match(&filename) {
                recursive_receipt_paths.push(path);
            } else if filename.starts_with("receipt_") {
                initial_receipt_path = Some(path);
            }
        }
    }

    recursive_receipt_paths.sort();

    let mut receipts = Vec::new();
    let mut input_output_chain: Vec<(Vec<u8>, Vec<String>, Vec<u8>)> = Vec::new();
    
    let initial_transformations;
    let initial_output_bytes;

    if !is_parallel {

    if let Some(init_path) = initial_receipt_path {
        let receipt_bytes = fs::read(init_path).expect("Failed to read initial receipt");
        let receipt: Receipt = bincode::deserialize(&receipt_bytes)
            .expect("Failed to deserialize initial receipt");

        let (transformations, output_bytes): (Vec<String>, Vec<u8>) =
            receipt.journal.decode().expect("Failed to decode initial journal");

        initial_output_bytes = output_bytes;
        initial_transformations = transformations;
        receipts.push(receipt);
    } else {
        panic!("Initial receipt not found (should be named like `receipt_<desc>.bin`)");
    }

    }
    else 
    {
        
        initial_transformations = vec![];
        initial_output_bytes = vec![];
    }

    for path in recursive_receipt_paths {
        let receipt_bytes = fs::read(path).expect("Failed to read recursive receipt");
        let receipt: Receipt = bincode::deserialize(&receipt_bytes)
            .expect("Failed to deserialize recursive receipt");

        let (input_image_bytes, transformations, output_bytes): (Vec<u8>, Vec<String>, Vec<u8>) =
            receipt
                .journal
                .decode()
                .expect("Failed to decode journal from recursive receipt");

        
        input_output_chain.push((input_image_bytes.clone(), transformations.clone(), output_bytes.clone()));
        

        receipts.push(receipt);
    }

    let mut builder = ExecutorEnv::builder();

    let mut env_builder = if !is_parallel {
        builder
            .write(&is_parallel).expect("Failed to write flag")
            .write(&(initial_transformations.clone(), initial_output_bytes.clone()))
            .expect("Failed to write initial output and transformations")
    } else {
        builder.write(&is_parallel).expect("Failed to write flag")
    };

    let num_recursive: u32 = input_output_chain.len() as u32;

    env_builder = env_builder.write(&num_recursive).expect("Failed to write recursive count");

    for (input_bytes, t, output_bytes) in &input_output_chain {
        env_builder = env_builder
            .write(&(input_bytes.clone(), t.clone(), output_bytes.clone()))
            .expect("Failed to write recursive transformation data");
    }
  
    for receipt in &receipts {
        env_builder = env_builder.add_assumption(receipt.clone());
    }

    let env = env_builder.build().expect("Failed to build zk env");

    let receipt = default_prover()
        .prove(env, COMPOSITION_ELF)
        .expect("Failed to generate composition proof")
        .receipt;

    receipt.verify(COMPOSITION_ID).expect("Verification failed");
    let receipt_bytes = bincode::serialize(&receipt).expect("Failed to serialize receipt");
    std::fs::write("composition_receipt.bin", receipt_bytes).expect("Failed to write receipt file");
}
