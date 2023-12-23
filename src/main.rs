#![allow(unused_variables, unused_imports, dead_code)]
use reqwest;
use std::env;
use std::fmt::format;
use std::process;
use tokio;
use web3::{db_connection, describe_table, ipfs_get, ipfs_push};
use serde_json::{json, Map, Value};

use merkletreers::merkletree::tree::MerkleTree;


#[tokio::main]
async fn main() {
      // checker().await;

    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("WARNING:: {err}");
        process::exit(1);
    });

     let host = Some(&args[1]);
     let username = Some(&args[2]);
     let password = Some(&args[3]);
     let db_name = Some(&args[4]);
     let table_name = &args[5];

     let mut json_data = json!({
        "table_name": table_name,
        "column": [],
        "row": []
     }); // Create an empty JSON array

    let pool = db_connection(host, username, password, db_name);
    let pool = match pool{
        Ok(val) => val,
        Err(err) => {
            println!("The err {:?}", err);
            println!("ERROR:: Please make sure your database credentials are valid");
            process::exit(1);
        }
    };
    println!(" ");
    println!("MySQL Database Connection Established Successfully!");
    println!(" ");

    //
    let mut conn = pool.get_conn().unwrap();

    let describe_query = format!("DESCRIBE {table_name}");
    let table_structure =  describe_table(&mut conn, describe_query, &mut json_data, "column");
    match table_structure{
        Ok(_) => (),
        Err(err) =>{
            println!("ERROR : {:#?}", err);
            process::exit(1);
        }
    }

    let get_query = format!("SELECT * FROM {table_name}");
    let get_data = describe_table(&mut conn, get_query, &mut json_data, "row");
    match get_data{
        Ok(_) => (),
        Err(err) =>{
            println!("ERROR : {:#?}", err);
            process::exit(1);
        }
    }

    let my_vec = json_data["row"].as_array().unwrap().clone();
    let mut cid_vec = vec![];
    for i in my_vec{
        let cid = ipfs_push(i).await;
        cid_vec.push(cid);

    }
    println!("CID of IPFS : {:?}", cid_vec);
    println!("");

    let mut vec_of_str: Vec<&str> = cid_vec.iter().map(|s| s.as_str()).collect();
    if vec_of_str.len() %2 != 0{
        vec_of_str.push("");
    }

    let tree = MerkleTree::new(vec_of_str);
    //
    println!("The Merkle Tree {:?}", tree);

    // Drop
    //
    //the mysql
    drop(conn);

    //ipfs_push().await;
}

struct Config {
    host: String,
    username: String,
    password: String,
    db_name: String,
    table_name: String,

}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 6 {
            return Err("Not Enough Arguments (Expected 5 arguments : host,username,password,db_name,table_name)");
        }
        let host = args[1].clone();
        let username = args[2].clone();
        let password = args[3].clone();
        let db_name = args[4].clone();
        let table_name = args[5].clone();

        Ok(Config { host,username,password,db_name,table_name })
    }
}



