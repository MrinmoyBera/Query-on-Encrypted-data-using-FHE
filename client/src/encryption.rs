use csv::{ReaderBuilder, WriterBuilder};
use tfhe::{FheInt8, ClientKey, FheInt32};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use tfhe::prelude::*;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use bincode;

//client key read from client key file
const CLIENT_KEY_FILE: &str = "/home/csc-pc4/client/client_key_file.bin";

/// Load the ClientKey from file
fn load_client_key() -> ClientKey {
    let mut file = File::open(CLIENT_KEY_FILE).expect("Failed to open client key file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Failed to read client key file");
    bincode::deserialize(&data).expect("Failed to deserialize client key")
}

/// Encrypt specified columns in a CSV file using TFHE
pub fn encrypt_csv(
    input_csv: &str,
    encrypted_csv: &str,
    target_columns: &[usize]
) {
    let keys = load_client_key();

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_csv)
        .expect("Failed to read input CSV");

    let mut writer = WriterBuilder::new()
        .from_path(encrypted_csv)
        .expect("Failed to create encrypted CSV file");

    for result in reader.records() {
        let record = result.expect("Failed to read record");
        let mut modified_record: Vec<String> = record.iter().map(|s| s.to_string()).collect();

        // Encrypt all target columns
        for &col_index in target_columns {
            if let Some(value) = record.get(col_index) {
                if let Ok(parsed_value) = value.parse::<i8>() {
                    let encrypted_value = FheInt8::try_encrypt(parsed_value, &keys)
                        .expect("Encryption failed");
                    let encrypted_bytes = bincode::serialize(&encrypted_value)
                        .expect("Serialization failed");
                    modified_record[col_index] = STANDARD.encode(encrypted_bytes);
                }
            }
        }

        writer.write_record(&modified_record).expect("Failed to write record");
    }

    writer.flush().expect("Failed to flush writer");
    println!("Encryption complete. Encrypted CSV saved to {}", encrypted_csv);
}



/// Encrypts an `i8` value using a serialized `ClientKey`.
///
/// # Arguments
/// * `client_key_path` - Path to the serialized `ClientKey` file.
/// * `value` - The plaintext value to encrypt.
///
/// # Returns
/// * `Option<FheInt8>` - Encrypted ciphertext or None if an error occurs.
pub fn encrypting_value(client_key_path: &str, value: i8) -> Option<FheInt8> {
    // Load the client key from file
    let mut file = File::open(client_key_path).ok()?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).ok()?;

    // Deserialize the client key
    let client_key: ClientKey = bincode::deserialize(&bytes).ok()?;

    // Encrypt the value
    FheInt8::try_encrypt(value, &client_key).ok()
}

// Float ----> Integer
pub fn enc_float_val(client_key_path: &str, float_value: f32, scale: f32) -> Option<FheInt32> {
    // Load the client key from file
    let mut file = File::open(client_key_path).ok()?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).ok()?;

    // Deserialize the client key
    let client_key: ClientKey = bincode::deserialize(&bytes).ok()?;

    // Scale the float to integer
    let scaled_value = (float_value * scale).round() as i32;

    // Encrypt the value
    FheInt32::try_encrypt(scaled_value, &client_key).ok()

}



/*

 let float_value = 3.141;
    let scale = 1000.0; // Retain 3 decimal places of precision

    match encryption::enc_float_val(client_key_path, float_value, scale) {
        Some(ciphertext) => {
            println!("Successfully encrypted the float value.");

            // Now decrypt the value
            match decryption::decrypt_float(client_key_path, &ciphertext, scale) {
                Some(decrypted_value) => {
                    println!("Decrypted float value: {:.4}", decrypted_value);
                }
                None => {
                    eprintln!("Decryption failed.");
                }
            }
        }
        None => {
            eprintln!("Encryption failed. Please check if the key file is valid.");
        }
    }


*/