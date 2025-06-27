use tfhe::prelude::*;
use tfhe::{FheBool, FheInt8, ServerKey, set_server_key};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::fs::File;
use std::io::Read;

pub fn sort_index_dsc(
    server_key_path: &str,
    enc_vec: Vec<String>,
    n: usize,
    encryp_0: FheInt8
) -> Option<Vec<FheInt8>> {
    // Load ServerKey from file
    let mut server_key_file = File::open(server_key_path).ok()?;
    let mut server_key_bytes = Vec::new();
    server_key_file.read_to_end(&mut server_key_bytes).ok()?;
    let server_key: ServerKey = bincode::deserialize(&server_key_bytes).ok()?;

    // Set the server key globally
    set_server_key(server_key.clone());

    // Decode input ciphertexts
    let mut enc_values: Vec<FheInt8> = Vec::new();
    for val in enc_vec {
        let encrypted_bytes = STANDARD.decode(val).ok()?;
        let encrypted_value: FheInt8 = bincode::deserialize(&encrypted_bytes).ok()?;
        enc_values.push(encrypted_value);
    }

    // Initialize comparison matrix
    let mut arr: Vec<Vec<FheInt8>> = vec![vec![encryp_0.clone(); n]; n];

    for i in 0..n {
        for j in 0..n {
            if i != j {
                let lt = enc_values[i].lt(&enc_values[j]);  // FheBool
                arr[i][j] = FheInt8::cast_from(lt);         // Convert to FheInt8
            }
        }
    }

    // Compute row-wise sum → encrypted sort indices
    let mut sort_indices: Vec<FheInt8> = Vec::new();
    for i in 0..n {
        let mut row_sum = encryp_0.clone();
        for j in 0..n {
            row_sum = &row_sum + &arr[i][j];
        }
        sort_indices.push(row_sum);
    }

    Some(sort_indices)
}



// For finding ascanding order of an encrypted array
pub fn sort_index_asc(
    server_key_path: &str,
    enc_vec: Vec<String>,
    n: usize,
    encryp_0: FheInt8,
    encrypt_1: FheInt8
) -> Option<Vec<FheInt8>> {
    // Load ServerKey from file
    let mut server_key_file = File::open(server_key_path).ok()?;
    let mut server_key_bytes = Vec::new();
    server_key_file.read_to_end(&mut server_key_bytes).ok()?;
    let server_key: ServerKey = bincode::deserialize(&server_key_bytes).ok()?;

    // Set the server key globally
    set_server_key(server_key.clone());

    // Decode input ciphertexts
    let mut enc_values: Vec<FheInt8> = Vec::new();
    for val in enc_vec {
        let encrypted_bytes = STANDARD.decode(val).ok()?;
        let encrypted_value: FheInt8 = bincode::deserialize(&encrypted_bytes).ok()?;
        enc_values.push(encrypted_value);
    }

    // Initialize comparison matrix
    let mut arr: Vec<Vec<FheInt8>> = vec![vec![encryp_0.clone(); n]; n];

    for i in 0..n {
        for j in (i+1)..n {
            let lt = enc_values[i].lt(&enc_values[j]);  // FheBool
            arr[i][j] = FheInt8::cast_from(lt);         // Convert to FheInt8
            arr[j][i] = &encrypt_1 - &arr[i][j];
        }
    }

    // Compute column-wise sum → encrypted sort indices
    let mut sort_indices: Vec<FheInt8> = Vec::new();
    for j in 0..n {
        let mut col_sum = encryp_0.clone();
        for i in 0..n {
            col_sum = &col_sum + &arr[i][j];
        }
        sort_indices.push(col_sum);
    }

    Some(sort_indices)
}





/*
// Sorting in descending order

// Encrypt the value 0 (used later for comparison)
    let enc_val = encrypt_value::encrypting_value(client_key_path, 0)
        .expect("Encryption failed");

    let dec_val = decrypt_val::decrypt_ciphertext(client_key_path, &enc_val);
    println!("dec_val: {:?}", dec_val);

    let start = Instant::now(); // Start timer

    // Perform the sorting and get encrypted sort indices
    let enc_sort_index = sorting_array::sort_index_dsc(server_key_path, values, 6, enc_val);

    // Decrypt the sort indices
    let mut sort_index = Vec::new();

    if let Some(enc_sort_vecs) = enc_sort_index {
        for enc_val in enc_sort_vecs {
                let dec = decrypt_val::decrypt_ciphertext(client_key_path, &enc_val);
                sort_index.push(dec);
        }
    } else {
        println!("No encrypted sort index was returned.");
    }
    println!("Decrypted sort index: {:?}", sort_index);
    
    let duration = start.elapsed(); // End timer
    println!("Time taken: {:.2?}", duration);
*/