# 🔐 Secure Query Execution on Encrypted Data using Homomorphic Encryption

This project demonstrates a secure end-to-end query execution system using Fully Homomorphic Encryption (FHE). It enables clients to query encrypted data stored in PostgreSQL without revealing sensitive information to the server.

---

## 📂 Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Key Generation & CSV Encryption/Decryption](#key-generation--csv-encryptiondecryption)
4. [Uploading Encrypted Data to PostgreSQL](#uploading-encrypted-data-to-postgresql)
5. [Connecting Rust Server to PostgreSQL](#connecting-rust-server-to-postgresql)
6. [Client Interaction Flow](#client-interaction-flow)
7. [Server-side Query Execution](#server-side-query-execution)
8. [Client-side Decryption](#client-side-decryption)

---

## 📘 Overview

This system enables **privacy-preserving query execution** on encrypted CSV data using **TFHE (Fully Homomorphic Encryption)**. Data remains encrypted during transmission, storage, and even while being queried.

---

## ✅ Prerequisites

* PostgreSQL (configured locally)
* pgAdmin for database management
* Rust (for server logic)
* A TFHE-based FHE library
* CSV files with plain or sensitive data

---

## 🔑 Key Generation & CSV Encryption/Decryption

The client begins by generating encryption keys and encrypting target columns in the CSV.

```rust
key_gen::initialize_keys();

let input_csv = "/home/csc-pc4/TFHE_hadoop/sample_data.csv";
let encrypted_csv = "/home/csc-pc4/TFHE_hadoop/encrypted.csv";
encryption::encrypt_csv(input_csv, encrypted_csv, &target_columns);

let decrypted_csv = "/home/csc-pc4/TFHE_hadoop/decrypted.csv";
decryption::decrypt_csv(encrypted_csv, decrypted_csv, &target_columns);
```

---

## 📤 Uploading Encrypted Data to PostgreSQL

### Step 1: Move Encrypted File

```bash
cp /home/csc-pc4/client/encrypted.csv /tmp/
chmod 644 /tmp/encrypted.csv
```

### Step 2: Configure pgAdmin

* **URL**: [http://127.0.0.1/pgadmin4](http://127.0.0.1/pgadmin4)
* **Server Name**: `testserver`
* **Host**: `localhost`
* **Port**: `5432`
* **Username**: `postgres`
* **Password**: `mbera@12345`

### Step 3: Create Table and Load Data

```sql
CREATE TABLE example_enc (
    name VARCHAR(20),
    salary TEXT,
    age TEXT,
    date_of_joining DATE
);

COPY example_enc FROM '/tmp/encrypted.csv' DELIMITER ',' CSV HEADER;
```

---

## 🦀 Connecting Rust Server to PostgreSQL

The Rust server connects to the database using Diesel ORM:

```rust
pub fn connect_to_db() -> Result<PgConnection, diesel::ConnectionError> {
    let database_url = "postgres://postgres:mbera123@localhost/query_fhe";
    PgConnection::establish(&database_url)
}
```

---

## 💻 Client Interaction Flow

The client provides input via terminal:

```text
Enter query_type:
Enter column_name:
Enter operation [e.g., greater_than or less_than] or leave blank:
Enter reference value or leave blank:
```

---

## 🖥️ Server-side Query Execution

The server performs FHE-based computation on encrypted columns and returns the **encrypted result** to the client.

---

## 🔓 Client-side Decryption

Once the encrypted result is received, the client uses the secret key to decrypt:
---

## 📎 Notes

* The encryption is done at column-level to enable secure filtering and comparison.
* Only the client holds the secret key; the server never sees plain data.
* PostgreSQL stores and serves encrypted records.
* All processing is done either in encrypted form or in trusted zones.

---
