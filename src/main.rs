#![allow(unused_variables,unused_imports,dead_code)]
use reqwest;
use serde_json::{json};
use tokio;
use web3::{db_connection, describe_table, ipfs_get, ipfs_push};
#[tokio::main]
async fn main() {
    let table_name = "students".to_string();
    let mut json_data = json!({
        "table_name": table_name,
        "column": [],
        "row": []
    }); // Create an empty JSON array

    let host = None; // Some("localhost");
    let username = None; //Some("sachinmurali");
    let password = None; //Some("sachin6624");
    let db_name = None; //Some("test");

    let pool = db_connection(host, username, password, db_name);
    println!("The mysql pool is {:?}", pool);

    let mut conn = pool.get_conn().unwrap();

    let describe_query = format!("DESCRIBE {table_name}");
    describe_table(&mut conn, describe_query, &mut json_data, "column");

    let get_query = format!("SELECT * FROM {table_name}");
    describe_table(&mut conn, get_query, &mut json_data, "row");

    println!("{:#?}", json_data);
    drop(conn);

    //ipfs_push().await;
}
