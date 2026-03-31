# farm-ledger
# 🌾 AniLedger: Decentralized Farm Transaction Ledger

**AniLedger** is a blockchain-based transparency tool designed for small-scale farmers in the Philippines. It eliminates the "middleman trust gap" by providing an immutable record of every crop sale, ensuring farmers are paid fairly and have a verifiable transaction history.

---

### 🔭 Vision
"Empowering Filipino farmers with blockchain technology—every harvest sale recorded permanently, transparently, and fairly. No more cheating, no more lost records."

### ❌ The Problem
Small-scale rice and vegetable farmers in rural provinces often lose significant income because local middlemen verbally dictate prices and leave unpaid balances. Without a physical or digital trail, farmers have no way to demand full payment or prove their creditworthiness to banks.

### ✅ The Solution
Using **Stellar's Soroban smart contracts**, AniLedger allows farmers and buyers to log transactions (crop type, weight, and price) on-chain. This creates a "Digital Receipt" that cannot be altered, deleted, or denied by either party.

---

### 🚀 Key Features
*   **Immutable Transaction Logging:** Records crop type, quantity, price, and date on the Stellar ledger.
*   **Dual Verification:** Both the farmer and the buyer must "sign" (approve) the transaction for it to be finalized.
*   **Permanent Proof of Sale:** Farmers maintain a lifetime history of sales, which can be used as a verifiable financial record for loans.
*   **Low-Cost Operation:** Leverages Stellar’s near-zero transaction fees, making it viable for even the smallest harvest.

---
### SAMPLE SITE
<img width="1823" height="1013" alt="image" src="https://github.com/user-attachments/assets/4eeb1efc-fe94-4b8c-bbd5-d091d3d3f0f9" />


### 📂 Project Structure

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
📂 Project Structure

<img width="425" height="478" alt="image" src="https://github.com/user-attachments/assets/e83225b1-8da1-49e3-a476-8c9d767baf3a" />


Contract ID: CD7KVIAGEMJCOZAPW23IWQLGTKGZXY2QS3AN7WPAOO4PB7DQERTDD6BY

Compiler Version: Solidity ^0.8.20

Network: Ethereum Sepolia
<img width="1815" height="997" alt="Screenshot 2026-03-31 165608" src="https://github.com/user-attachments/assets/c7cf9bae-13ea-4145-af6d-c1c697ddbedd" />
<img width="1853" height="994" alt="Screenshot 2026-03-31 165638" src="https://github.com/user-attachments/assets/92efd37d-af9d-44b9-9491-0812c8465eac" />

### 🛠️ Stellar Features Used

*   **Soroban Smart Contracts:** Used to implement the core agreement logic and manage the decentralized state storage for all farm transactions.
*   **Custom Assets (Planned):** Future integration for issuing "Digital Harvest Receipts" as unique, verifiable tokens on the Stellar network.
*   **Trustlines:** Utilized to ensure the secure and compliant handling of payments when settling transactions via **USDC** or other stablecoins.

---

### 📖 Core Logic Overview

The smart contract is built around a robust `Transaction` struct. To ensure maximum security and mutual agreement:

1.  **State Management:** A transaction is initialized in a `Pending` state.
2.  **Dual-Authorization:** The record only moves to the `Completed` state once both the `farmer_address` and the `buyer_address` have successfully invoked the approval function.
3.  **Financial Accuracy:** The contract automatically calculates the settlement value on-chain to prevent manual entry errors:

$$Total Payment = Quantity \times UnitPrice$$

---

### 🏆 Why This Wins (Hackathon Criteria)

*   **Real-World Impact:** Directly addresses the "middleman" exploitation and lack of financial records affecting millions of Filipino farmers.
*   **Stellar-Centric:** Specifically engineered to take advantage of Stellar’s **ultra-low fees** and **sub-second finality**, making micro-ledger entries economically feasible.
*   **Demo-Ready:** Features a streamlined, end-to-end flow from "Input Sale" to "On-Chain Receipt" that can be fully demonstrated to judges in **under 2 minutes**.


