/*
Destructured Problem:

Section 1:

There is a certain table in MySQL. You need to get the names of their columns
and their type. Then get the cell values line by line and generate JSON where the
table name, columns and cell value are indicated.

Section 2:
Interfere with İPFS, receive CİD and build a chain of hashes for each line
consisting of hash cells and a chain of
lines that will ultimately be tied to the İPNS hash.

Section 3:
The program must be written in
Rust. Contain instructions for compilation and receive arguments for
connecting to the MySQL database. It is possible to attach a compiled file for
Linux.”

*/

use std::collections::HashMap;
//mysql://sql12672069:aVeqNDSepn@sql12.freesqldatabase.com:3306/sql12672069?prefer_socket=false
use mysql::{OptsBuilder, Pool, PooledConn};
use mysql::prelude::Queryable;
use mysql::{prelude::FromRow, Row};
use serde_json::{json, Map, Value};
use std::borrow::Cow;

fn main() {
    let table_name = "students".to_string();
    let mut db_fields = Vec::<String>::new();
    let mut json_data = json!({
        "table_name": table_name,
        "column": [],
        "row": []
    }); // Create an empty JSON array
    let host = None;// Some("localhost");
    let username = None;//Some("sachinmurali");
    let password = None;//Some("sachin6624");
    let db_name = None;//Some("test");

    let opts = db_connection(host,username,password,db_name);
    let pool = Pool::new(opts).expect("Failed to create MySQL connection pool");
    println!("The pool is {:?}", pool);



    let mut conn = pool.get_conn().unwrap();
    // let column_query = format!("SELECT COLUMN_NAME, DATA_TYPE FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = {:?} AND TABLE_NAME = {:?}", "sql12672069",table_name);
    // let columns: Vec<(String, String)> = conn.fetch_all().unwrap();

    //
    let describe_query = format!("DESCRIBE {table_name}");
    describe_table(&mut conn, describe_query, &mut json_data, "column", &mut db_fields);

     let get_query = format!("SELECT * FROM {table_name}");
    describe_table(&mut conn, get_query, &mut json_data, "row", &mut db_fields);





    println!("{:#?}", json_data);
    drop(conn);
}


fn db_connection(hostname:Option<&str>, username:Option<&str>, password:Option<&str>, db_name:Option<&str>)-> OptsBuilder{
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(hostname.unwrap_or("sql12.freesqldatabase.com")))
        .user(Some(username.unwrap_or("sql12672069")))
        .pass(Some(password.unwrap_or("aVeqNDSepn")))
        .db_name(Some(db_name.unwrap_or("sql12672069")));

    opts
}


fn describe_table(conn:&mut PooledConn, mysql_query:String, json_data: &mut Value, tag: &str, db_fields: &mut Vec<String>){
    match conn.query::<mysql::Row, _>(mysql_query) {

        Ok(result) => {
            // Iterate over the fetched rows and process them
            for (index,row) in result.iter().enumerate() {
                // println!("The len : {:?}", row.len());
                // println!("The len : {:?}", row);

                 if tag == "row"{
                     println!("The len : {:?} ", row);
                     // let mut json_object= HashMap::<Cow<str>,String>::new();
                     // let mut json_object = Map::<String,String>::new();
                     let mut json_object = Map::new();

                     for (index, column) in row.columns_ref().iter().enumerate() {
                         let column_name = column.name_str().to_string();
                         let column_value = Value::String(row.get(index).unwrap());
                         // println!("Column Name: {:?}, Column Value: {:?}", column_name, column_value);
                          json_object.insert(column_name, column_value);
                     }
                     let result = json!(json_object);

                      println!("{}", result);
                     // let x =  serde_json::to_string_pretty(&result).unwrap()
                     json_data[tag].as_array_mut().unwrap().push(result);




                 }else{

                     let name: String  = row.get(0).unwrap();
                     let data_type: String = row.get(1).unwrap();

                     //println!("Name: {} | Type: {}", name, data_type);

                     let row_json = json!({
                            "name": name,
                            "type": data_type,
                     });

                     db_fields.push(name);

                     // println!("working: {:?} ", row_json);

                     // Add the row JSON to the array
                     json_data[tag].as_array_mut().unwrap().push(row_json);

                 }




            }

            // println!("The len is {:#?}", json_data);


        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}



