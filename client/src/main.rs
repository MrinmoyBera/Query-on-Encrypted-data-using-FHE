mod encryption;
mod decryption;
mod key_gens;
use tfhe::{FheInt8, FheBool};
use std::net::TcpStream;
use std::io::{self,Write, Read};
use serde::Serialize;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use bincode;

#[derive(Serialize)]
struct QueryRequest {
    query_type: String,
    column_name: String,
    operation: Option<String>, //"less_than", "greater_than", "equal"
    ref_val: Option<i8>, // or Option<FheInt8> if encrypted
}


const client_key_path: &str = "/home/csc-pc4/client/client_key_file.bin";

fn main() {

    let mut input = String::new();

    // Get query_type
    print!("Enter query_type: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let query_type = input.trim().to_string();
    input.clear();

    // Get column_name
    print!("Enter column_name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let column_name = input.trim().to_string();
    input.clear();
    
    
    // Get operation (optional)
    print!("Enter operation (e.g., greater_than) or leave blank: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let operation = {
        let op = input.trim();
        if op.is_empty() || op.eq_ignore_ascii_case("none") {
            None
        } else {
            Some(op.to_string())
        }
    };
    input.clear();

    // Get ref_val (optional)
    print!("Enter reference value (i8) or leave blank: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let ref_val = {
        let val = input.trim();
        if val.is_empty() || val.eq_ignore_ascii_case("none") {
            None
        } else {
            Some(val.parse::<i8>().expect("Invalid number"))
        }
    };
    input.clear();
    
    // Construct request
    let request = QueryRequest {
        query_type,
        column_name,
        operation,
        ref_val,
    };
    
    // Established TCP Connection
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Cannot connect to server");

    // Convert to JSON and send
    let json_request = serde_json::to_string(&request).unwrap();
    stream.write_all(json_request.as_bytes()).unwrap();

   
    // Read response
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).unwrap();
    
    if query_type == "sort_by_asc" || query_type == "sort_by_dsc" {
        let ciphertext_vec : Vec < FheInt8 > = bincode :: deserialize (& buffer ) .
        unwrap () ;
        let decrypted_values : Vec < i8 > = ciphertext_vec
        . iter ()
        . map (| ct | decryption :: decrypt_ciphertext ( client_key_path , ct )
        . expect (" Decryption failed ") )
        . collect () ;
        println !(" Decrypted values : {:?}" , decrypted_values ) ;
    } else if query_type == "where" {
        let ciphertext_vec : Vec < FheBool > = bincode :: deserialize (& buffer ) .
        unwrap () ;
        let decrypted_bools : Vec < bool > = ciphertext_vec . iter ()
        . map (| ct | decryption :: decrypt_bool_ciphertext ( client_key_path , ct
        ) )
        . collect () ;
        println !(" Decrypted booleans : {:?}" , decrypted_bools ) ;
    } else if query_type == "avg" {
        let ( encrypted_value , count ) : ( FheInt8 , usize ) = bincode :: deserialize
        (& buffer ) . unwrap () ;
        let decrypted_opt = decryption :: decrypt_ciphertext ( client_key_path , &
        encrypted_value ) ;
        match decrypted_opt {
        Some ( decrypted_sum ) = > {
        let average = decrypted_sum as f64 / count as f64 ;
        println !(" Decrypted sum : {}" , decrypted_sum ) ;
        println !(" Average : {:.2}" , average ) ;
        }
        None = > {
        println !(" Failed to decrypt received data .") ;
        }
        }
    } else if query_type == "min" || query_type == "max" {
        let encrypted_value : FheInt8 = bincode :: deserialize (& buffer ) . unwrap ();
        let decrypted_opt = decryption :: decrypt_ciphertext ( client_key_path , &
        encrypted_value ) ;
        match decrypted_opt {
        Some ( decrypted_max ) = > {
        println !(" Decrypted max or max : {}" , decrypted_max ) ;
        }
        None = > {
        println !(" Failed to decrypt received data .") ;
        }
        }
    } else {
        println("Please Input valid query!")
    }

    
    
    
}