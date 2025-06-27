use tfhe::prelude::*; // Import necessary traits
use tfhe::{FheInt8, ClientKey, FheBool, FheInt32};
use csv::{ReaderBuilder, WriterBuilder};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::fs::File;
use std::io::Read;
use bincode;

const CLIENT_KEY_FILE: &str = "/home/csc-pc4/client/client_key_file.bin";

/// Load the ClientKey from file
fn load_client_key() -> ClientKey {
    let mut file = File::open(CLIENT_KEY_FILE).expect("Failed to open client key file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Failed to read client key file");
    bincode::deserialize(&data).expect("Failed to deserialize client key")
}

/// Decrypt specified columns in an encrypted CSV file using TFHE
pub fn decrypting_csv_file(
    encrypted_csv: &str,
    decrypted_csv: &str,
    target_columns: &[usize]
) {
    let keys = load_client_key();

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(encrypted_csv)
        .expect("Failed to read encrypted CSV");

    let mut writer = WriterBuilder::new()
        .from_path(decrypted_csv)
        .expect("Failed to write decrypted CSV");

    for result in reader.records() {
        let record = result.expect("Failed to read record");
        let mut modified_record: Vec<String> = record.iter().map(|s| s.to_string()).collect();

        // Decrypt all target columns
        for &col_index in target_columns {
            if let Some(value) = record.get(col_index) {
                if let Ok(decoded_bytes) = STANDARD.decode(value) {
                    let encrypted_value: FheInt8 = bincode::deserialize(&decoded_bytes)
                        .expect("Failed to deserialize");
                    let decrypted_value: i8 = encrypted_value.decrypt(&keys);
                    modified_record[col_index] = decrypted_value.to_string();
                }
            }
        }

        writer.write_record(&modified_record).expect("Failed to write record");
    }

    writer.flush().expect("Failed to flush writer");
    println!("Decryption complete. Decrypted CSV saved to {}", decrypted_csv);
}


/// Decrypts a given `FheInt8` ciphertext using a `ClientKey` loaded from file.
///
/// # Arguments
/// * `client_key_path` - Path to the serialized `ClientKey` file.
/// * `ciphertext` - The encrypted value (`FheInt8`) to decrypt.
///
/// # Returns
/// * `Option<i8>` - The decrypted `i8` value, or `None` on failure.
pub fn decrypt_ciphertext(client_key_path: &str, ciphertext: &FheInt8) -> Option<i8> {
    // Load the client key from file
    let mut file = File::open(client_key_path).ok()?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).ok()?;

    // Deserialize the client key
    let client_key: ClientKey = bincode::deserialize(&bytes).ok()?;

    // Decrypt the ciphertext
    let decrypted_value: i8 = ciphertext.decrypt(&client_key);
    Some(decrypted_value)
}

pub fn decrypt_bool_ciphertext(client_key_path: &str, ct: &FheBool) -> bool {
    // Load client key from file
    let client_key_bytes = std::fs::read(client_key_path).expect("Failed to read client key");
    let client_key: ClientKey = bincode::deserialize(&client_key_bytes).expect("Deserialization failed");

    ct.decrypt(&client_key)
}


/// Decrypts an `FheInt32` ciphertext and rescales it to a `f32` value using the scale factor.
pub fn decrypt_float(client_key_path: &str, ciphertext: &FheInt32, scale: f32) -> Option<f32> {
    // Load the client key from file
    let mut file = File::open(client_key_path).ok()?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).ok()?;

    // Deserialize the client key
    let client_key: ClientKey = bincode::deserialize(&bytes).ok()?;

    // Decrypt the ciphertext to i32
    let decrypted_value: i32 = ciphertext.decrypt(&client_key);

    // Rescale to approximate float
    let float_value = decrypted_value as f32 / scale;

    Some(float_value)
}