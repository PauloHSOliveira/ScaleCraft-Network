// File: main.rs

mod blockchain;
mod wallet;

use crate::wallet::Transaction;
use crate::blockchain::Blockchain;

fn main() {
    let difficulty = 5;
    let mut blockchain = Blockchain::create(difficulty);

    let alice_address = wallet::WalletAddress::new(String::from("Alice"));

    let bob_address = wallet::WalletAddress::new(String::from("Alice"));

    let transaction = Transaction {
        sender: alice_address.clone(),
        receiver: bob_address.clone(),
        amount: 18.0,
    };

    blockchain.add_transaction(transaction);
    blockchain.mine_pending_transactions(alice_address.clone());

    // Create a new token
    blockchain.create_token("SCN", 1000);

    // Create accounts
    blockchain.create_account("Alice");
    blockchain.create_account("Bob");

    // Mint tokens for Alice
    blockchain.mint_tokens("Alice", 500, "SCN").unwrap();

    // Transfer 200 SCN tokens from Alice to Bob
    blockchain
        .transfer_tokens("Alice", "Bob", 200, "SCN")
        .unwrap();

    // Print blockchain information and check if it's valid
    println!("Blockchain: {:?}", blockchain);
    println!("Is Blockchain Valid? {}", blockchain.is_chain_valid());
}

