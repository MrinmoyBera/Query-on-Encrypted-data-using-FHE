mod max_finder;
mod average;
mod min_finder;
mod sorting_array;
mod compare;
mod query_example;
use tfhe::{FheBool, FheInt8};
mod server;
use bincode;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct QueryRequest {
    query_type: String,
    column_name: String,
    operation:Option<String>,
    ref_val: Option<i8>, 
}

const SERVER_KEY_PATH: &str = "/home/csc-pc4/client/server_key_file.bin";
const client_key_path: &str = "/home/csc-pc4/client/client_key_file.bin";


fn handle_client(mut stream: TcpStream) {
    // connect to the postgresql server
    let mut conn = server::connect_to_db().unwrap();
    let mut buffer = [0; 2048];
    let size = stream.read(&mut buffer).unwrap();
    let request_str = String::from_utf8_lossy(&buffer[..size]);
    let request: QueryRequest = serde_json::from_str(&request_str).expect("Invalid JSON");

    println!("Received request: {:?}", request);
    let enc_0 = query_example::encrypting_value(client_key_path, 0);
    let enc_1 = query_example::encrypting_value(client_key_path, 1);
    match request.query_type.as_str() {
        "avg" => {
            let response = query_example::avg_enc(&mut conn, SERVER_KEY_PATH, &request.column_name);
            if let Some((cipher, count)) = response {
                let serialized = bincode::serialize(&(cipher, count)).unwrap();
                stream.write_all(&serialized).unwrap();
                println!("Sent encrypted result.");
            }
        },
        "max" => {
            let response = query_example::max_enc(&mut conn, SERVER_KEY_PATH, &request.column_name);
            if let Some(cipher) = response {
                let serialized = bincode::serialize(&cipher).unwrap();
                stream.write_all(&serialized).unwrap();
                println!("Sent encrypted result.");
            }
        },
        "min" => {
            let response = query_example::min_enc(&mut conn, SERVER_KEY_PATH, &request.column_name);
            if let Some(cipher) = response {
                let serialized = bincode::serialize(&cipher).unwrap();
                stream.write_all(&serialized).unwrap();
                println!("Sent encrypted result.");
            }
        },
        "order_by_asc" =>{
            // Unwrap Option values
            let enc_0_unwrapped = enc_0.expect("enc_0 is None");
            let enc_1_unwrapped = enc_1.expect("enc_1 is None");

            let response = query_example::order_by_asc(
                &mut conn,
                SERVER_KEY_PATH,
                &request.column_name,
                enc_0_unwrapped,
                enc_1_unwrapped,
            );

            if let Some(cipher) = response {
                let serialized = bincode::serialize(&cipher).unwrap();
                stream.write_all(&serialized).unwrap();
                println!("Sent encrypted result.");
            }
        },
        "order_by_dsc" =>{
            // Unwrap Option values
            let enc_0_unwrapped = enc_0.expect("enc_0 is None");

            let response = query_example::order_by_dsc(
                &mut conn,
                SERVER_KEY_PATH,
                &request.column_name,
                enc_0_unwrapped
            );

            if let Some(cipher) = response {
                let serialized = bincode::serialize(&cipher).unwrap();
                stream.write_all(&serialized).unwrap();
                println!("Sent encrypted result.");
            }
        },
        /*
        "where_query" =>{
            if let Some(val) = request.ref_val{
                if let Some(refference_val) = query_example::encrypting_value(client_key_path, val) {
                    let response = query_example::where_query(&mut conn, SERVER_KEY_PATH, refference_val,
                        &request.operation, &request.column_name);
                
                    if let Some(cipher) = response {
                        let serialized = bincode::serialize(&cipher).unwrap();
                        stream.write_all(&serialized).unwrap();
                        println!("Sent encrypted result.");
                    }
                } else {
                    eprintln!("Encryption failed or value was None");
                }
            }else {
                eprintln!("Missing reference value for where_query");
            }
        },*/
        _ => {
        eprintln!("Unknown query type: {}", request.query_type);
        // optionally send an error response to client
        }

    }    
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| handle_client(stream));
    }
}