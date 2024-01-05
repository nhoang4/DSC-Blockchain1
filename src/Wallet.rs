#![allow(non_snake_case)]




    use std::path::{Path, PathBuf};
    use openssl::rsa::Rsa;
    use openssl::pkey::PKey;
    use std::{env, io, str};
    use std::fs::File;
    use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
    use serde_yaml;


     pub struct Wallet
    {
        pub balance: i32,
        pub public_key: String,
        pub private_key: String
    }

    //blake3
//4 funcs
    //write into yaml file, conains pub, priv, balance: create wallet
    //both store in dsckey.yaml
    //balance: read yaml file
    //transaction: ...
    //key: read from file, print public, private

    impl Wallet {

        // pub fn create_Wallet() -> Result<(), Box<dyn std::error::Error>> {
        //     if file_exists() == false {
        //         let balance = 0;
        //         let public_key = Rsa::generate(2048).unwrap();
        //
        //         let private_key = PKey::from_rsa(public_key.clone()).unwrap();
        //
        //         let pub_key: Vec<u8> = private_key.public_key_to_pem().unwrap();
        //
        //         println!("{:?}", str::from_utf8(pub_key.as_slice()).unwrap());
        //
        //         let content = Wallet{
        //             balance,
        //             public_key: str::from_utf8(pub_key.as_slice()).unwrap().parse().unwrap(),
        //             private_key: String::from(""),
        //         };
        //
        //         let file = File::create("dsc_key.yaml")?;
        //         serde_yaml::to_writer(file, &content);
        //         println!("Successfully create a new Wallet.");
        //         Ok(())
        //
        //
        //     }
        //     else { println!("Wallet Already Exists."); Ok(())}
        // }

        pub fn view_Balance() -> io::Result<()>  {
            if file_exists(){
                let currdir =  env::current_dir().unwrap().as_path().join("dsc_key.yaml");
                let file  = File::open(currdir)?;
                let reader = BufReader::new(file);

                if let Some(thirdLine) = reader.lines().nth(2){
                    let line = thirdLine;
                    match line {
                        Ok(res) =>  println!("{}", res),
                        Err(err) => {
                            eprintln!("Error: {}", err);
                        }
                    }

                }
                Ok(())
            }
            else {
                println!("File does not exist1: {}", "dsc_key.yaml");
                Ok(())
            }
        }

        pub fn view_key() -> io::Result<()>{
            if file_exists() {
                let currdir =  env::current_dir().unwrap().as_path().join("dsc_key.yaml");
                let file  = File::open(currdir)?;
                let mut lines = BufReader::new(file).lines();

                let first_line = lines.next().ok_or_else(|| {
                    Error::new(ErrorKind::InvalidData, "File is empty")
                })??;

                let second_line = lines.next().unwrap_or_else(|| Ok(String::new()))?;

                println!("{}\n{}", first_line, second_line);
                Ok(())
            }
            else {
                println!("File does not exist2: {}", "dsc_key.yaml");
                Ok(())
            }
        }

        pub fn transaction(){

        }



    }
    fn file_exists() -> bool {
        let path = Path::new("dsc_key.yaml");
        path.exists() && path.is_file()
    }

