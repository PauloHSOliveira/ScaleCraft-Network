use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct WalletAddress(String);

#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: WalletAddress,
    pub receiver: WalletAddress,
    pub amount: f64,
}

impl Hash for Transaction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.sender.0.hash(state);
        self.receiver.0.hash(state);
        self.amount.to_bits().hash(state);
    }
}

impl WalletAddress {
    pub fn new(address: String) -> Self {
        Self(address)
    }
}
