use tfhe::prelude::*;
use tfhe::{FheInt8, ServerKey, set_server_key, FheBool};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::fs::File;
use std::io::Read;
use bincode;


pub fn compare_enc(
    server_key_path: &str,
    enc_vec: Vec<String>,        // Encrypted vector (Base64 strings)
    reference_value: FheInt8,    // Encrypted reference value
    operation: &str              // "less_than", "greater_than", "equal"
) -> Option<Vec<FheBool>> {

    // Load ServerKey from file
    let mut server_key_file = File::open(server_key_path).ok()?;
    let mut server_key_bytes = Vec::new();
    server_key_file.read_to_end(&mut server_key_bytes).ok()?;
    let server_key: ServerKey = bincode::deserialize(&server_key_bytes).ok()?;

    // Set the server key globally
    set_server_key(server_key);

    let mut compare_result: Vec<FheBool> = Vec::new();

    for enc_val_str in enc_vec {
        // Decode and deserialize ciphertext
        let encrypted_bytes = STANDARD.decode(&enc_val_str).ok()?;
        let enc_val: FheInt8 = bincode::deserialize(&encrypted_bytes).ok()?;

        // Perform comparison
        let cmp: FheBool = match operation {
            "less_than" => enc_val.lt(&reference_value),
            "greater_than" => enc_val.gt(&reference_value),
            "equal" => enc_val.eq(&reference_value),
            _ => return None, // Invalid operation
        };

        compare_result.push(cmp);
    }

    Some(compare_result)
}





/*
//For checking comparison functionalities
    // Start timer
    let start = Instant::now();

    // Perform the comparison
    let compare_result = compare::compare_enc(server_key_path, enc_salary, enc_1, "less_than");

    // Prepare vector to store decrypted results
    let mut compare_res_dec = Vec::new();

    if let Some(compare_res_vecs) = compare_result {
        for enc_bool in compare_res_vecs {
            // Decrypt the casted result
            let dec = decrypt_val::decrypt_bool_ciphertext(client_key_path, &enc_bool);
            compare_res_dec.push(dec);
        }
    } else {
        println!("No encrypted comparison result was returned.");
    }

    // Show decrypted results
    println!("Decrypted comparison results: {:?}", compare_res_dec);

    // End timer
    let duration = start.elapsed();
    println!("Time taken: {:.2?}", duration);

*/