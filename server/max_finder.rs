use tfhe::prelude::*;
use tfhe::{FheInt8, ServerKey, set_server_key};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::fs::File;
use std::io::Read;

/// Computes the homomorphic sum and count of a vector of base64-encoded FheInt8 ciphertexts.
///
/// # Arguments
///
/// * `server_key_path` - Path to the file containing the serialized ServerKey.
/// * `enc_vec` - A vector of base64-encoded ciphertexts (`FheInt8`).
///
/// # Returns
///
/// * maximum value of the given array
pub fn array_max(
    server_key_path: &str,
    enc_vec: Vec<String>,
) -> Option<FheInt8> {
    // Load ServerKey from file
    let mut server_key_file = File::open(server_key_path).ok()?;
    let mut server_key_bytes = Vec::new();
    server_key_file.read_to_end(&mut server_key_bytes).ok()?;
    let server_key: ServerKey = bincode::deserialize(&server_key_bytes).ok()?;

    // Set the server key globally
    set_server_key(server_key.clone());

    let mut max: Option<FheInt8> = None;

    for val in enc_vec {
        let encrypted_bytes = STANDARD.decode(val).ok()?;
        let encrypted_value: FheInt8 = bincode::deserialize(&encrypted_bytes).ok()?;

        max = Some(match max {
            Some(current_max) => current_max.max(&encrypted_value),
            None => encrypted_value,
        });
    }
    max
}



/*
//For calculating max of an array

let start = Instant::now(); // Start timer

    let max_value = max_finder::array_max(server_key_path, values);
    
    match max_value {
        Some(ref max_val) => {
            // Decrypt the max value
            let decrypted_max = decrypt_val::decrypt_ciphertext(client_key_path, max_val);
            match decrypted_max {
                Some(decrypted_value) => {
                    println!("Decrypted max value: {}", decrypted_value);
                }
                None => {
                    println!("Failed to decrypt max value.");
                }
            }
        }
        None => {
            println!("No max value computed.");
        }
    }

    let duration = start.elapsed(); // End timer

    println!("Time taken: {:.2?}", duration);

*/