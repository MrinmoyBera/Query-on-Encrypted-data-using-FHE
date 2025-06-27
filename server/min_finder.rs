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
/// * minimum value of the given array
pub fn array_min(
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

    let mut min: Option<FheInt8> = None;

    for val in enc_vec {
        let encrypted_bytes = STANDARD.decode(val).ok()?;
        let encrypted_value: FheInt8 = bincode::deserialize(&encrypted_bytes).ok()?;

        min = Some(match min {
            Some(current_min) => current_min.min(&encrypted_value),
            None => encrypted_value,
        });
    }
    min
}



/*
//For Finding minimum of a column we are run following thinks in main functions

    let start = Instant::now(); // Start timer

    let min_value = min_finder::array_min(server_key_path, values);
    
    match min_value {
        Some(ref min_val) => {
            // Decrypt the min value
            let decrypted_min = decrypt_val::decrypt_ciphertext(client_key_path, min_val);
            match decrypted_min {
                Some(decrypted_value) => {
                    println!("Decrypted minimum value: {}", decrypted_value);
                }
                None => {
                    println!("Failed to decrypt minimum value.");
                }
            }
        }
        None => {
            println!("No minimum value computed.");
        }
    }

    let duration = start.elapsed(); // End timer

    println!("Time taken: {:.2?}", duration);

*/