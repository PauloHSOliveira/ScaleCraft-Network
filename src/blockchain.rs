// File: blockchain.rs

use std::collections::HashMap;
use sha2::{Sha256, Digest};
use crate::wallet::{WalletAddress, Transaction, self};
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
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
}

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: f64,
    pub tokens: HashMap<String, Token>,
    pub accounts: HashMap<String, Account>,
}

#[derive(Debug)]
pub struct Token {
    pub symbol: String,
    pub total_supply: u64,
    pub balances: HashMap<String, u64>,
}

#[derive(Debug)]
pub struct Account {
    pub address: String,
    pub balance: u64,
    pub token_balances: HashMap<String, u64>,
}

impl Token {
    // Transfer tokens from one account to another
    fn transfer(&mut self, from: &str, to: &str, amount: u64) -> Result<(), String> {
        let from_balance = self.balances.get(from).cloned().unwrap_or(0);
        if from_balance < amount {
            return Err("Insufficient balance".to_string());
        }

        let to_balance = self.balances.get(to).cloned().unwrap_or(0);

        self.balances.insert(from.to_string(), from_balance - amount);
        self.balances.insert(to.to_string(), to_balance + amount);

        Ok(())
    }
}

impl Blockchain {
    // Create a new blockchain with a genesis block
    pub fn new(difficulty: usize) -> Self {
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
            tokens: HashMap::new(),
            accounts: HashMap::new(),
        }
    }

    pub fn create(difficulty: usize) -> Self {
        Self::new(difficulty)
    }

    // Add a new transaction to the list of pending transactions
    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    // Mine pending transactions and add a new block to the blockchain
    pub fn mine_pending_transactions(&mut self, miner_address: WalletAddress) {
        let system_address = wallet::WalletAddress::new(String::from("System"));

        let reward_transaction = Transaction {
            sender: system_address.clone(),
            receiver: miner_address.clone(),
            amount: self.mining_reward,
        };
        self.pending_transactions.push(reward_transaction);

        let previous_hash = self.blocks.last().unwrap().hash.clone();
        let new_block = self.create_block(previous_hash);
        self.blocks.push(new_block);

        self.pending_transactions.clear();
    }

    // Create a new block with pending transactions
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
        block
    }

    // Validate the entire blockchain
    pub fn is_chain_valid(&self) -> bool {
        let mut prev_hash = String::from("genesis_block_hash"); // Change this to match the hash of the actual genesis block

        for block in &self.blocks {
            if block.previous_hash != prev_hash {
                return false;
            }

            let calculated_hash = block.calculate_hash();
            if block.hash != calculated_hash {
                return false;
            }

            prev_hash = block.hash.clone();
        }

        true
    }

    // Create a new token
    pub fn create_token(&mut self, symbol: &str, total_supply: u64) {
        let token = Token {
            symbol: symbol.to_string(),
            total_supply,
            balances: HashMap::new(),
        };
        self.tokens.insert(symbol.to_string(), token);
    }

    // Create a new account with tokens
    pub fn create_account(&mut self, address: &str) {
        let account = Account {
            address: address.to_string(),
            balance: 0,
            token_balances: HashMap::new(),
        };
        self.accounts.insert(address.to_string(), account);
    }

    // Mint tokens and assign them to an account
    pub fn mint_tokens(&mut self, account_address: &str, amount: u64, token_symbol: &str) -> Result<(), String> {
        let account = self.accounts.get_mut(account_address).ok_or("Account not found")?;
        let token = self.tokens.get_mut(token_symbol).ok_or("Token not found")?;

        if token.total_supply < amount {
            return Err("Mint amount exceeds total supply".to_string());
        }

        token.balances
            .entry(account_address.to_string())
            .and_modify(|balance| *balance += amount)
            .or_insert(amount);

        account.token_balances
            .entry(token_symbol.to_string())
            .and_modify(|balance| *balance += amount)
            .or_insert(amount);

        token.total_supply -= amount;

        Ok(())
    }


    // Transfer tokens between accounts
   pub fn transfer_tokens(
        &mut self,
        sender_address: &str,
        receiver_address: &str,
        amount: u64,
        token_symbol: &str,
    ) -> Result<(), String> {
        let sender_account = self.accounts
            .get_mut(sender_address)
            .ok_or("Sender account not found")?;
        
        let sender_balance = sender_account
            .token_balances
            .get(token_symbol)
            .cloned()
            .unwrap_or(0);
        
        if sender_balance < amount {
            return Err("Insufficient balance".to_string());
        }

        let token = self.tokens.get_mut(token_symbol).ok_or("Token not found")?;

       
        token.transfer(sender_address, receiver_address, amount)?;

        let sender_token_balance_entry = sender_account.token_balances
            .entry(token_symbol.to_string())
            .or_insert(0);
        *sender_token_balance_entry -= amount;

        let receiver_account = self.accounts
            .get_mut(receiver_address)
            .ok_or("Receiver account not found")?;
        let receiver_token_balance_entry = receiver_account.token_balances
            .entry(token_symbol.to_string())
            .or_insert(0);
        *receiver_token_balance_entry += amount;

        Ok(())
    } 
    
}