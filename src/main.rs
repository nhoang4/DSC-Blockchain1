use std::path::Path;


mod Wallet;

fn main() {
    // let w = Wallet::module_a::Wallet::create_Wallet();
    // let a = Wallet::module_a::Wallet::view_Balance();
    // let k1 = Wallet::module_a::Wallet::view_key();

    let w =  Wallet::Wallet::view_Balance();
    let a = Wallet::Wallet::view_key();

}
