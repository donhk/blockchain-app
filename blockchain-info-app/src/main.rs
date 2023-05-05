#[macro_use]
extern crate serde_derive;

mod blockchain_info;
mod blockchain_address;
mod blockchain_status;
mod blockchain_transactions;

use crate::blockchain_status::BlockchainStatus;
use crate::blockchain_address::BlockchainAddress;
use crate::blockchain_transactions::BlockchainTransaction;

fn main() {
    let blockchain_status = blockchain_info::blockchain_status_request();
    println!("Querying {} - chain {}",
             &blockchain_status.blockbook.coin,
             &blockchain_status.backend.chain
    )
}
