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
/// * `Option<(FheInt8, usize)>` - Encrypted sum and count, or `None` if input is empty or failed.

pub fn compute_sum_and_count(
    server_key_path: &str,
    enc_vec: Vec<String>,
) -> Option<(FheInt8, usize)> {
    // Load ServerKey from file
    let mut server_key_file = File::open(server_key_path).ok()?;
    let mut server_key_bytes = Vec::new();
    server_key_file.read_to_end(&mut server_key_bytes).ok()?;
    let server_key: ServerKey = bincode::deserialize(&server_key_bytes).ok()?;

    // Set the server key globally
    set_server_key(server_key.clone());

    let mut sum: Option<FheInt8> = None;
    let mut count = 0;

    for val in enc_vec {
        let encrypted_bytes = STANDARD.decode(val).ok()?;
        let encrypted_value: FheInt8 = bincode::deserialize(&encrypted_bytes).ok()?;

        sum = Some(match sum {
            Some(current_sum) => current_sum + &encrypted_value,
            None => encrypted_value,
        });

        count += 1;
    }

    // Return sum and count if any values were processed
    if let Some(total) = sum {
        Some((total, count))
    } else {
        None
    }
}

