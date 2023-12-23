#![allow(unused_variables,unused_imports,dead_code)]
use mysql::prelude::Queryable;
use mysql::{OptsBuilder, Pool, PooledConn};
use pinata_sdk::{PinByJson, PinataApi};
use serde_json::{json, Map, Value};
use std::collections::HashMap;

pub fn db_connection(
    hostname: Option<&String>,
    username: Option<&String>,
    password: Option<&String>,
    db_name: Option<&String>,
) ->  Result<Pool, mysql::Error>{
    let opts = OptsBuilder::new()
        .ip_or_hostname(hostname)
        .user(username)
        .pass(password)
        .db_name(db_name);

    let pool = Pool::new(opts)?;
    Ok(pool)
}

pub async fn ipfs_get() {
    // IPFS gateway URL
    let gateway_url = "https://ipfs.io/ipfs/";

    // IPFS CID (Content Identifier) of the data you want to retrieve
    let cid = "QmXTXCP6bWUEakz9eMbvQpKLtRR8uiCh8ygLAoTMm27u9A";

    // Concatenate the gateway URL and CID to form the complete URL
    let url = format!("{}{}", gateway_url, cid);

    // Make a GET request to the IPFS gateway
    let response = reqwest::get(&url)
        .await
        .expect("Failed to retrieve data from IPFS");

    // Check if the request was successful (status code 200)
    if response.status().is_success() {
        // Read and print the response body (IPFS data)
        let content = response.text().await.expect("Failed to read response body");
        println!("Retrieved data from IPFS:\n{}", content);
    } else {
        println!(
            "Failed to retrieve data from IPFS. Status code: {}",
            response.status()
        );
    }
}

pub async fn ipfs_push() {
    let api = PinataApi::new(
        "53eccefc8154167d1f21",
        "16e94c021d48176b52cc59fc071ba8833a2d75eeb48273de8e2ed60e30d8a5c8",
    )
    .unwrap();

    // HashMap derives serde::Serialize
    let mut json_data = HashMap::new();
    json_data.insert("Creator", "Billa");

    let result = api.pin_json(PinByJson::new(json_data)).await;

    if let Ok(pinned_object) = result {
        let hash = pinned_object.ipfs_hash;
        println!("Hash : {:?}", hash)
    }
}

pub fn describe_table(
    conn: &mut PooledConn,
    mysql_query: String,
    json_data: &mut Value,
    tag: &str,
)->  Result<(), mysql::Error> {
    match conn.query::<mysql::Row, _>(mysql_query) {
        Ok(result) => {
            // Iterate over the fetched rows and process them
            for (index, row) in result.iter().enumerate() {
                if tag == "row" {
                    let mut json_object = Map::new();
                    for (index, column) in row.columns_ref().iter().enumerate() {
                        let column_name = column.name_str().to_string();
                        let column_value = Value::String(row.get(index).unwrap());
                        json_object.insert(column_name, column_value);
                    }
                    let result = json!(json_object);
                    json_data[tag].as_array_mut().unwrap().push(result);
                } else {
                    let name: String = row.get(0).unwrap();
                    let data_type: String = row.get(1).unwrap();

                    let row_json = json!({
                           "name": name,
                           "type": data_type,
                    });

                    // Add the row JSON to the array
                    json_data[tag].as_array_mut().unwrap().push(row_json);
                }
            }
             Ok(())
        }
        Err(err) => {
            Err(err)
        }
    }
}
