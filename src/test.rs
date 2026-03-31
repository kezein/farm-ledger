#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

#[test]
// Test 1 - Happy path: adding a transaction succeeds end-to-end
fn test_add_transaction_success() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FarmLedger);
    let client = FarmLedgerClient::new(&env, &contract_id);

    let count = client.add_transaction(
        &String::from_str(&env, "Juan dela Cruz"),
        &String::from_str(&env, "Rice"),
        &100,
        &50,
        &String::from_str(&env, "2026-03-31"),
    );

    // Should return 1 since it's the first transaction
    assert_eq!(count, 1);
}

#[test]
// Test 2 - Edge case: adding transaction with zero quantity
fn test_add_transaction_zero_quantity() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FarmLedger);
    let client = FarmLedgerClient::new(&env, &contract_id);

    let count = client.add_transaction(
        &String::from_str(&env, "Maria Santos"),
        &String::from_str(&env, "Corn"),
        &0,
        &30,
        &String::from_str(&env, "2026-03-31"),
    );

    // Transaction still records but quantity is 0
    assert_eq!(count, 1);
}

#[test]
// Test 3 - State verification: storage reflects correct state after transaction
fn test_storage_state_after_transaction() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FarmLedger);
    let client = FarmLedgerClient::new(&env, &contract_id);

    client.add_transaction(
        &String::from_str(&env, "Pedro Reyes"),
        &String::from_str(&env, "Mango"),
        &200,
        &80,
        &String::from_str(&env, "2026-03-31"),
    );

    // Verify storage has exactly 1 transaction
    let count = client.get_count();
    assert_eq!(count, 1);

    // Verify the transaction data is correct
    let transactions = client.get_transactions();
    let tx = transactions.get(0).unwrap();
    assert_eq!(tx.crop, String::from_str(&env, "Mango"));
    assert_eq!(tx.quantity, 200);
}

#[test]
// Test 4 - Multiple transactions: ledger grows correctly
fn test_multiple_transactions() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FarmLedger);
    let client = FarmLedgerClient::new(&env, &contract_id);

    client.add_transaction(
        &String::from_str(&env, "Juan"),
        &String::from_str(&env, "Rice"),
        &100,
        &50,
        &String::from_str(&env, "2026-03-31"),
    );

    client.add_transaction(
        &String::from_str(&env, "Maria"),
        &String::from_str(&env, "Corn"),
        &150,
        &40,
        &String::from_str(&env, "2026-03-31"),
    );

    // Should have 2 transactions now
    assert_eq!(client.get_count(), 2);
}

#[test]
// Test 5 - Get transactions: returns empty list when no transactions exist
fn test_get_transactions_empty() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FarmLedger);
    let client = FarmLedgerClient::new(&env, &contract_id);

    // No transactions added yet
    let transactions = client.get_transactions();
    assert_eq!(transactions.len(), 0);
}