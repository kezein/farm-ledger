# farm-ledger

🌾 Farm Ledger: Decentralized Agricultural Management
Farm Ledger is a blockchain-based accounting and supply chain solution designed to bring transparency and immutable record-keeping to the agricultural sector. By leveraging smart contracts, it allows farmers, distributors, and retailers to track crop lifecycles, log financial transactions, and verify the origin of produce.

🔭 Vision
To empower small-to-medium scale farmers with enterprise-grade financial tools, eliminating the "middleman" trust gap and providing a verifiable credit history through on-chain data.

🚀 Key Features
Immutable Crop Logging: Record planting dates, fertilizer usage, and harvest yields that cannot be tampered with.

Financial Ledger: Track sales and operational expenses directly tied to specific crop batches.

Supply Chain Provenance: Generate unique IDs for produce batches to allow end-consumers to verify origin.

Role-Based Access: Distinct permissions for Farmers (data entry), Auditors (verification), and Buyers (procurement).

📂 Project Structure
A clean, modular structure following industry standards (Hardhat/Foundry style):

Plaintext
.
├── contracts/            # Solidity Smart Contracts
│   ├── FarmLedger.sol    # Main logic for ledger and supply chain
│   ├── AccessControl.sol # Role management
│   └── Libs/             # Custom structs and logic libraries
├── scripts/              # Deployment and interaction scripts
├── test/                 # Comprehensive unit tests
├── README.md             # Project documentation
└── hardhat.config.js     # Network and compiler configurations
⛓️ Deployed Contract Details
The contract is currently live on the [Insert Network Name, e.g., Sepolia/Polygon] Testnet.

Contract ID: CD7KVIAGEMJCOZAPW23IWQLGTKGZXY2QS3AN7WPAOO4PB7DQERTDD6BY

Compiler Version: Solidity ^0.8.20

Network: Ethereum Sepolia
<img width="1815" height="997" alt="Screenshot 2026-03-31 165608" src="https://github.com/user-attachments/assets/c7cf9bae-13ea-4145-af6d-c1c697ddbedd" />
<img width="1853" height="994" alt="Screenshot 2026-03-31 165638" src="https://github.com/user-attachments/assets/92efd37d-af9d-44b9-9491-0812c8465eac" />



