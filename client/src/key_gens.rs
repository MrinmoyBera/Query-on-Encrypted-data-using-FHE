use tfhe::{generate_keys, ConfigBuilder};
use std::fs::File;
use std::io:: Write;
use bincode;

const CLIENT_KEY_FILE: &str = "/home/csc-pc4/client/client_key_file.bin";
const SERVER_KEY_FILE: &str = "/home/csc-pc4/client/server_key_file.bin";

/// Save data to a file
fn save_to_file(filename: &str, data: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(data)?;
    Ok(())
}

/// Generate or load TFHE keys and store them if not already stored
pub fn initialize_keys() {
        let config = ConfigBuilder::default().build();
        let (client_key, server_key) = generate_keys(config);

        save_to_file(CLIENT_KEY_FILE, &bincode::serialize(&client_key).expect("Client key serialization failed"))
            .expect("Failed to save client key");
        save_to_file(SERVER_KEY_FILE, &bincode::serialize(&server_key).expect("Server key serialization failed"))
            .expect("Failed to save server key");

        println!("New keys generated and saved.");
    }