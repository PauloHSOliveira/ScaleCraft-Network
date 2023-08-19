use std::hash::{Hash, Hasher};
use sha2::{Sha256, Digest};

#[derive(Debug)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
}

#[derive(Debug)]
struct Block {
    index: u32,
    timestamp: u64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

impl Hash for Transaction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sender.hash(state);
        self.receiver.hash(state);
        self.amount.to_bits().hash(state);
    }
}

impl Block {
    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{:?}", self));
        format!("{:x}", hasher.finalize())
    }

    fn pow(&mut self, difficulty: usize) {
        let target_prefix = "0".repeat(difficulty);
        while !self.hash.starts_with(&target_prefix) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }

    fn print_info(&self) {
        println!("Block Info:");
        println!("Index: {}", self.index);
        println!("Timestamp: {}", self.timestamp);
        println!("Transactions: {:?}", self.transactions);
        println!("Previous Hash: {}", self.previous_hash);
        println!("Hash: {}", self.hash);
        println!("Nonce: {}", self.nonce);
    }
}

#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    fn new(difficulty: usize) -> Self {
        Blockchain {
            blocks: Vec::new(),
            difficulty,
        }
    }

    fn add_block(&mut self, transactions: Vec<Transaction>, previous_hash: String) {
        let index = self.blocks.len() as u32;
        let timestamp = 0; // Set the timestamp (will be adjusted later)
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(), // Set the hash (will be calculated later)
            nonce: 0, // Set the initial nonce
        };

        block.pow(self.difficulty); // Execute Proof of Work
        block.print_info();
        self.blocks.push(block);
    }

    fn is_chain_valid(&self) -> bool {
        let mut prev_hash = String::from("previous_hash_here");

        for block in &self.blocks {
            if block.previous_hash != prev_hash {
                return false;
            }

            if block.hash != block.calculate_hash() {
                return false;
            }

            prev_hash = block.hash.clone();
        }

        true
    }
}

fn main() {
    let difficulty = 4;
    let mut blockchain = Blockchain::new(difficulty);

    let transactions = vec![Transaction {
        sender: String::from("Alice"),
        receiver: String::from("Bob"),
        amount: 18.0,
    }];
    let previous_hash = String::from("genesis_block_hash"); // Change this to a unique value
    blockchain.add_block(transactions, previous_hash);

    let transactions2 = vec![Transaction {
        sender: String::from("Bob"),
        receiver: String::from("Charlie"),
        amount: 10.0,
    }];
    let previous_hash2 = blockchain.blocks.last().unwrap().hash.clone(); // Get hash from the last block
    blockchain.add_block(transactions2, previous_hash2);

    println!("Blockchain: {:?}", blockchain);
    println!("Is Blockchain Valid? {}", blockchain.is_chain_valid());
}