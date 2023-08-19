use std::hash::{Hash, Hasher};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
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
    pending_transactions: Vec<Transaction>,
    mining_reward: f64,
}

impl Blockchain {
    fn new(difficulty: usize) -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            transactions: Vec::new(),
            previous_hash: String::from("genesis_block_hash"),
            hash: String::from("genesis_block_hash"),
            nonce: 0,
        };

        Blockchain {
            blocks: vec![genesis_block],
            difficulty,
            pending_transactions: Vec::new(),
            mining_reward: 10.0,
        }
    }

    fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    fn mine_pending_transactions(&mut self, miner_address: &str) {
        let reward_transaction = Transaction {
            sender: String::from("system"),
            receiver: String::from(miner_address),
            amount: self.mining_reward,
        };
        self.pending_transactions.push(reward_transaction);

        let previous_hash = self.blocks.last().unwrap().hash.clone();
        let new_block = self.create_block(previous_hash);
        self.blocks.push(new_block);

        self.pending_transactions.clear();
    }

    fn create_block(&self, previous_hash: String) -> Block {
        let index = self.blocks.len() as u32;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let mut block = Block {
            index,
            timestamp,
            transactions: self.pending_transactions.clone(),
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };

        block.pow(self.difficulty);
        block.print_info();
        block
    }

    fn is_chain_valid(&self) -> bool {
        let mut prev_hash = String::from("genesis_block_hash");

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
    let difficulty = 5;
    let mut blockchain = Blockchain::new(difficulty);

    blockchain.add_transaction(Transaction {
        sender: String::from("Alice"),
        receiver: String::from("Bob"),
        amount: 18.0,
    });

    blockchain.mine_pending_transactions("miner_address");

    blockchain.add_transaction(Transaction {
        sender: String::from("Bob"),
        receiver: String::from("Charlie"),
        amount: 10.0,
    });

    blockchain.mine_pending_transactions("miner_address");

    println!("Blockchain: {:?}", blockchain);
    println!("Is Blockchain Valid? {}", blockchain.is_chain_valid());
}
