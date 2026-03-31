#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, vec, Env, String, Vec};

// Storage key for accessing the list of transactions
#[contracttype]
pub enum DataKey {
    Transactions,
}

// Struct representing a single farm transaction
#[contracttype]
#[derive(Clone)]
pub struct FarmTransaction {
    pub farmer: String,   // Name of the farmer
    pub crop: String,     // Type of crop sold
    pub quantity: u32,    // Quantity in kilograms
    pub price: u32,       // Price per kg
    pub date: String,     // Date of transaction e.g. "2026-03-31"
}

#[contract]
pub struct FarmLedger;

#[contractimpl]
impl FarmLedger {
    // Add a new farm transaction to the ledger
    pub fn add_transaction(
        env: Env,
        farmer: String,
        crop: String,
        quantity: u32,
        price: u32,
        date: String,
    ) -> u32 {
        // Get existing transactions or start with empty list
        let mut transactions: Vec<FarmTransaction> = env
            .storage()
            .persistent()
            .get(&DataKey::Transactions)
            .unwrap_or(vec![&env]);

        // Create new transaction
        let new_tx = FarmTransaction {
            farmer,
            crop,
            quantity,
            price,
            date,
        };

        // Add to list and save back to storage
        transactions.push_back(new_tx);
        env.storage()
            .persistent()
            .set(&DataKey::Transactions, &transactions);

        // Return total number of transactions recorded
        transactions.len()
    }

    // Get all recorded farm transactions
    pub fn get_transactions(env: Env) -> Vec<FarmTransaction> {
        env.storage()
            .persistent()
            .get(&DataKey::Transactions)
            .unwrap_or(vec![&env])
    }

    // Get total number of transactions recorded
    pub fn get_count(env: Env) -> u32 {
        let transactions: Vec<FarmTransaction> = env
            .storage()
            .persistent()
            .get(&DataKey::Transactions)
            .unwrap_or(vec![&env]);
        transactions.len()
    }
}
#[cfg(test)]
mod test;