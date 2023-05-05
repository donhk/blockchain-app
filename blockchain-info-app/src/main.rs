#[macro_use]
extern crate serde_derive;

mod blockchain_info;
mod blockchain_address;
mod blockchain_status;
mod blockchain_transactions;

use {
    crate::blockchain_status::BlockchainStatus,
    crate::blockchain_address::BlockchainAddress,
    crate::blockchain_transactions::BlockchainTransaction,
    dotenv,
    std::{io, thread, time},
};

fn blockchain_info_app(address: &str) {
    let blockchain_status: BlockchainStatus = blockchain_info::blockchain_status_request();
    println!("Querying {} - chain {}",
             &blockchain_status.blockbook.coin,
             &blockchain_status.backend.chain
    );
    let blockchain_address: BlockchainAddress = blockchain_info::blockchain_address_request(address);
    println!("Analyzing transactions for Bitcoin address {}", &blockchain_address.address);
    let sleep_time = time::Duration::from_millis(2500);
    thread::sleep(sleep_time);
}

fn main() {
    let entered_address = dotenv::var("WALLET").expect("Failed to read WALLET");
    blockchain_info_app(&entered_address);
}
