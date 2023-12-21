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

//  mysql://sql12672069:aVeqNDSepn@sql12.freesqldatabase.com:3306/sql12672069?prefer_socket=false
use mysql::{OptsBuilder, Pool};
use mysql::prelude::Queryable;
use serde_json::{json, Value};


fn main() {
    let mut json_data = json!([]); // Create an empty JSON array
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some("sql12.freesqldatabase.com"))
        .user(Some("sql12672069"))
        .pass(Some("aVeqNDSepn"))
        .db_name(Some("sql12672069"));

    let pool = Pool::new(opts).expect("Failed to create MySQL connection pool");

    println!("The pool is {:?}", pool);

    let mut conn = pool.get_conn().unwrap();

    match conn.query::<mysql::Row, _>("SELECT * FROM students") {
        Ok(result) => {
            println!("{:?}", result.len());

            // Iterate over the fetched rows and process them
            for row in result {
                let id: i32 = row.get(0).unwrap();
                let name: String = row.get(1).unwrap();

                // ... (process other columns)

                println!("ID: {} | Name: {}", id, name);

            }

        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }

    // Release the connection back to the pool
    drop(conn);
}

