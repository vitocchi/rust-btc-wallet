extern crate bitcoin;
extern crate bitcoin_wallet;

//use bitcoin::network::constants::Network;
use bitcoin::network::constants::Network;
use bitcoin_wallet::account::Account;
use bitcoin_wallet::account::AccountAddressType;
use bitcoin_wallet::account::MasterAccount;
use bitcoin_wallet::account::MasterKeyEntropy;
use bitcoin_wallet::account::Unlocker;

fn main() {
    let mut master =
        MasterAccount::new(MasterKeyEntropy::Sufficient, Network::Bitcoin, "pass").unwrap();
    let mut unlocker = Unlocker::new_for_master(&master, "pass").unwrap();
    let account = Account::new(&mut unlocker, AccountAddressType::P2PKH, 0, 0, 10).unwrap();
    master.add_account(account);
    println!("{:#?}", master);
    println!("Hello, world!");
}
