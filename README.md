# farm-ledger
# 🌾 AniLedger: Decentralized Farm Transaction Ledger

**AniLedger** is a blockchain-based transparency tool designed for small-scale farmers in the Philippines. It eliminates the "middleman trust gap" by providing an immutable record of every crop sale, ensuring farmers are paid fairly and have a verifiable transaction history.

---

### Vision
"Empowering Filipino farmers with blockchain technology—every harvest sale recorded permanently, transparently, and fairly. No more cheating, no more lost records."

### The Problem
Small-scale rice and vegetable farmers in rural provinces often lose significant income because local middlemen verbally dictate prices and leave unpaid balances. Without a physical or digital trail, farmers have no way to demand full payment or prove their creditworthiness to banks.

### The Solution
Using **Stellar's Soroban smart contracts**, AniLedger allows farmers and buyers to log transactions (crop type, weight, and price) on-chain. This creates a "Digital Receipt" that cannot be altered, deleted, or denied by either party.

---

## Target Users
*   **Who:** Smallholder Rice and Onion Farmers earning **₱10,000–₱25,000** per cycle. These users currently rely on fragile paper records and lack formal credit history.
*   **Where:** Agricultural regions in the **Philippines**, specifically **Nueva Ecija** and **Central Luzon**.
*   **Why:** To eliminate "Information Asymmetry." Farmers need a tamper-proof record of their harvest yields to qualify for low-interest government loans and micro-financing, moving away from predatory local lenders.

---

## Core Feature (MVP)
**The "Harvest Trust" Transaction Flow**
*   **User Action:** Farmer logs into the mobile interface and enters harvest data (e.g., `Crop: Red Onion`, `Weight: 500kg`).
*   **On-Chain Action:** A Soroban Smart Contract calls the `record_harvest` function, storing the Farmer's ID, Crop Type, Weight, and a Block Timestamp into persistent storage.
*   **Result:** The system generates a **Harvest Receipt Hash**. This provides an immutable, third-party verifiable record of production that acts as a "Digital Resume" for the farm.

## Core Logic Overview

The smart contract is built around a robust `Transaction` struct. To ensure maximum security and mutual agreement:

1.  **State Management:** A transaction is initialized in a `Pending` state.
2.  **Dual-Authorization:** The record only moves to the `Completed` state once both the `farmer_address` and the `buyer_address` have successfully invoked the approval function.
3.  **Financial Accuracy:** The contract automatically calculates the settlement value on-chain to prevent manual entry errors:

$$Total Payment = Quantity \times UnitPrice$$

---

### Key Features
*   **Immutable Transaction Logging:** Records crop type, quantity, price, and date on the Stellar ledger.
*   **Dual Verification:** Both the farmer and the buyer must "sign" (approve) the transaction for it to be finalized.
*   **Permanent Proof of Sale:** Farmers maintain a lifetime history of sales, which can be used as a verifiable financial record for loans.
*   **Low-Cost Operation:** Leverages Stellar’s near-zero transaction fees, making it viable for even the smallest harvest.

---

### Why This Wins

*   This project bridges the gap between rural agriculture and global finance by providing unbanked Filipino farmers with an immutable "Proof of Productivity" on Stellar, turning physical harvests into verifiable digital assets for credit-building. It perfectly aligns with Stellar’s mission of financial inclusion by using Soroban smart contracts to create a scalable, low-cost ledger that is ready to integrate with local anchors like PHPC for instant, real-world utility.

---

### Optional Edge: Local Anchor Integration
This project is designed to integrate with **Stellar Anchors (like Coins.ph/PHPC)**. 
**The Enhancement:** Beyond just logging data, the platform allows for **Pre-Harvest Financing**. A cooperative can send **PHPC (Philippine Peso Stablecoin)** to an escrow contract, which is released to the farmer automatically once the harvest weight is logged and verified on-chain.

---
### SAMPLE SITE
<img width="1825" height="1009" alt="image" src="https://github.com/user-attachments/assets/9d149314-c828-452d-951f-cf10642322f2" />


### 📂 Project Structure

```
.
├── js/                   # Frontend Application Logic
│   └── app.js            # Stellar SDK integration & UI event handling
├── src/                  # Soroban Smart Contract (Rust)
│   ├── lib.rs            # Core ledger logic & transaction functions
│   └── test.rs           # Automated unit tests for contract validation
├── target/               # Compiled Wasm binaries (Build output)
├── .gitignore            # Specifies files to ignore (e.g., /target)
├── Cargo.toml            # Rust manifest and contract dependencies
├── index.html            # Main User Interface (DApp)
├── LICENSE               # Open-source licensing
└── README.md             # Project documentation and setup guide
```

## Smart Contract Functions

| Function | Description |
|---|---|
| `add_transaction` | Records a new farm crop sale on-chain |
| `get_transactions` | Returns all recorded transactions |
| `get_count` | Returns total number of transactions |

---

## Deployed Contract Details

- **Network:** Stellar Testnet
- **Contract ID:** `CD7KVIAGEMJCOZAPW23IWQLGTKGZXY2QS3AN7WPAOO4PB7DQERTDD6BY`
- **Explorer:** [View on Stellar Expert](https://stellar.expert/explorer/testnet/contract/CD7KVIAGEMJCOZAPW23IWQLGTKGZXY2QS3AN7WPAOO4PB7DQERTDD6BY)

---
**Contract ID: CD7KVIAGEMJCOZAPW23IWQLGTKGZXY2QS3AN7WPAOO4PB7DQERTDD6BY**

<img width="1815" height="997" alt="Screenshot 2026-03-31 165608" src="https://github.com/user-attachments/assets/c7cf9bae-13ea-4145-af6d-c1c697ddbedd" />
<img width="1853" height="994" alt="Screenshot 2026-03-31 165638" src="https://github.com/user-attachments/assets/92efd37d-af9d-44b9-9491-0812c8465eac" />

### Stellar Features Used

*   **Soroban Smart Contracts:** Used to implement the core agreement logic and manage the decentralized state storage for all farm transactions.
*   **Custom Assets (Planned):** Future integration for issuing "Digital Harvest Receipts" as unique, verifiable tokens on the Stellar network.
*   **Trustlines:** Utilized to ensure the secure and compliant handling of payments when settling transactions via **USDC** or other stablecoins.

---
##  Live Demo

[https://kezein.github.io/farm-ledger](https://kezein.github.io/farm-ledger)

## Running Tests
```bash
cargo test
```

All 5 tests pass:
- ✅ Happy path: transaction records successfully
- ✅ Edge case: zero quantity transaction
- ✅ State verification: storage reflects correct data
- ✅ Multiple transactions: ledger grows correctly
- ✅ Empty state: returns empty list with no transactions


## Future Improvements

- Add farmer wallet verification for authenticated submissions
- SMS-based input system for farmers with limited media literacy
- Barangay cooperative officer portal for assisted data entry
- Mobile app with Filipino language support

## Developer

Built by Kezia Lorein Villegas for the RiseIn Stellar Philippines UniTour Bootcamp — March 2026


---


