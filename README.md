
# DeveloperTask-UMMA

A brief description of what this project does and who it's for


## Prerequsite

Install latest Rust lang ( follow this link to install rust)

```bash
`https://www.rust-lang.org/tools/install`

```




## Installation

Clone this repo and run

```bash
  cd ./web3/
```
-  Use this command to build the rust crates in production mode

- `cargo build --release`

Its gonna take some time to build it in release mode, so sit back.
Once the build is completed, use the following command to
run the release build

If you dont have mysql db credentials , don't worry just simply run the following command, it will run the executable with my database creds

` cargo run --release sql12.freesqldatabase.com sql12672069 aVeqNDSepn sql12672069 students `

OR

if you only have the binary executable with you, then run the following command. open a command line and go to folder , where thee executable is kept and run the following command

`
./web3 sql12.freesqldatabase.com sql12672069 aVeqNDSepn sql12672069 students    
`

if you want to run with your own database creds the replace the following fields with your database

-  `cargo run --release {host} {username} {db_password} {db_name} {table_name}`
   or
- `./web3  {host} {username} {db_password} {db_name} {table_name}  `

- host : mysql host
- username : mysql username
- db_password : mysql password
- db_name : mysql database name
- table_name : mysql table name



