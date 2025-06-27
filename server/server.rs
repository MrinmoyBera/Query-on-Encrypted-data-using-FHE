use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{Text, Integer};
use diesel::RunQueryDsl;
use tfhe::integer::backward_compatibility::server_key;

/// Establish and return a PostgreSQL connection
pub fn connect_to_db() -> Result<PgConnection, diesel::ConnectionError> {
    // Directly specify the connection URL here
    let database_url = "postgres://postgres:mbera123@localhost/query_fhe";
    PgConnection::establish(database_url)
}



/// Create a table named `sample_data`
pub fn create_table(conn: &mut PgConnection) {
    let create_sql = r#"
        CREATE TABLE IF NOT EXISTS sample_data (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL
        );
    "#;

    match sql_query(create_sql).execute(conn) {
        Ok(_) => println!("✅ Table created (or already exists)."),
        Err(e) => eprintln!("❌ Error creating table: {}", e),
    }
}

/// Insert a new row into `sample_data`
pub fn insert_data(conn: &mut PgConnection, name: &str) {
    let insert_sql = format!("INSERT INTO sample_data (name) VALUES ('{}');", name);

    match sql_query(insert_sql).execute(conn) {
        Ok(_) => println!("✅ Data inserted: {}", name),
        Err(e) => eprintln!("❌ Error inserting data: {}", e),
    }
}



#[derive(Debug, QueryableByName)]
struct ResultRow {
    #[sql_type = "Text"]
    value: String,
}

/// Get values from any column (Text or Integer) as Strings
pub fn get_column_data(column: &str, conn: &mut PgConnection) -> Vec<String> {
    // Use casting to ensure all types become TEXT
    let query = format!("SELECT {}::text AS value FROM example_enc", column);

    let result = sql_query(query)
        .load::<ResultRow>(conn);

    match result {
        Ok(rows) => rows.into_iter().map(|r| r.value).collect(),
        Err(e) => {
            eprintln!("Error: {}", e);
            vec![]
        }
    }
}


/// Get values from any column (Text or Integer) as Strings
pub fn column_data_from_table(table_name: &str, column: &str, conn: &mut PgConnection) -> Vec<String> {
    // Use casting to ensure all types become TEXT
    let query = format!("SELECT {}::text AS value FROM {}", column, table_name);

    let result = sql_query(query)
        .load::<ResultRow>(conn);

    match result {
        Ok(rows) => rows.into_iter().map(|r| r.value).collect(),
        Err(e) => {
            eprintln!("Error: {}", e);
            vec![]
        }
    }
}







/*

// connect to the postgresql server
let mut conn = server::connect_to_db().unwrap();

*/