# DeveloperTask-UMMA
- Prerequisite
  - install  cargo 1.74.1 
  - if you dont have rust installed, check below link
[    - `https://www.rust-lang.org/tools/install`
](https://www.rust-lang.org/tools/install)
- Clone the repo and enter into the directory
- `cd web3/`
-  Use this command to build the rust crates in production mode
  - `cargo build --release`
- Its gonna take some time to build it in release mode, so sit back
- Once the build is completed, use the following command to 
run the release build
-  `cargo run -- {host} {username} {db_password} {db_name} {table_name}`
- host : mysql host
- username : mysql username
- db_password : mysql password
- db_name : mysql database name
- table_name : mysql table name

If you dont have mysql db credentials , don't worry just simply run the following command, it will run the executable with my database creds

` cargo run -- sql12.freesqldatabase.com sql12672069 aVeqNDSepn sql12672069 students `


