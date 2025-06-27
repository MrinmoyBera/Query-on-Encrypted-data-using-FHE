use crate::compare::compare_enc;
use crate::server::get_column_data;
use crate::max_finder::array_max;
use crate::min_finder::array_min;
use crate::sorting_array::{sort_index_asc, sort_index_dsc};
use crate::average::compute_sum_and_count;
use tfhe::prelude::*;
use tfhe::{FheInt8, FheBool,ClientKey};
use std::fs::File;
use std::io::Read;
use diesel::pg::PgConnection;

//For finding maximum value of an column
pub fn max_enc(conn: &mut PgConnection,  server_key_path: &str, column: &str)-> Option<FheInt8>{

    let enc_vec = get_column_data(column, conn);
    let max_value = array_max(server_key_path, enc_vec);
    max_value
}

//For finding minimum value
pub fn min_enc(conn: &mut PgConnection,  server_key_path: &str, column: &str)-> Option<FheInt8>{

    let enc_vec = get_column_data(column, conn);
    let min_value = array_min(server_key_path, enc_vec);
    min_value
}

//For finding average value of an column 
pub fn avg_enc(
    conn: &mut PgConnection,
    server_key_path: &str,
    column: &str
) -> Option<(FheInt8, usize)> {
    // Get encrypted vector from the specified column
    let enc_vec = get_column_data(column, conn);

    // Compute encrypted sum and count
    let (sum_cipher, count) = compute_sum_and_count(server_key_path, enc_vec)
    .expect("Failed to compute sum and count");

    // Return the result
    Some((sum_cipher, count))
}



//For sorting a column in ascanding order
pub fn order_by_asc(
    conn: &mut PgConnection,
    server_key_path: &str,
    column: &str,
    enc_0: FheInt8,
    enc_1: FheInt8
) -> Option<Vec<FheInt8>> {
    // Get encrypted vector from the specified column
    let enc_vec = get_column_data(column, conn);

    // Get the number of elements
    let n = enc_vec.len();

    // Perform the sorting and get encrypted sort indices
    let enc_sort_index = sort_index_asc(server_key_path, enc_vec, n, enc_0,enc_1);

    enc_sort_index
}



//For sorting a column in ascanding order
pub fn order_by_dsc(
    conn: &mut PgConnection,
    server_key_path: &str,
    column: &str,
    enc_0: FheInt8
) -> Option<Vec<FheInt8>> {
    // Get encrypted vector from the specified column
    let enc_vec = get_column_data(column, conn);

    // Get the number of elements
    let n = enc_vec.len();

    // Perform the sorting and get encrypted sort indices
    let enc_sort_index = sort_index_dsc(server_key_path, enc_vec, n, enc_0);

    enc_sort_index
}


//For perform where query
pub fn where_query(
    conn: &mut PgConnection,
    server_key_path: &str,   
    reference_value: FheInt8,    // Encrypted reference value
    operation: &str,              // "less_than", "greater_than", "equal"
    column: &str
) -> Option<Vec<FheBool>>{
    // Get encrypted vector from the specified column
    let enc_vec = get_column_data(column, conn);
    // Perform the comparison
    let compare_result = compare_enc(server_key_path, enc_vec, reference_value, operation);
    compare_result
}


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
