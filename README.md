# farm-ledger
PROJECT NAME
AniLedger

PROBLEM
Small-scale rice and vegetable farmers in rural Philippine provinces lose up to 30% of their income because local middlemen verbally dictate prices and leave unpaid balances, leaving farmers with no verifiable proof to demand full payment.

SOLUTION
Farmers and buyers agree on a crop weight and price via a simple mobile web app, generating an immutable, timestamped Soroban smart contract record that serves as permanent, undeniable proof of the transaction.

STELLAR FEATURES USED

Soroban smart contracts

USDC transfers (optional for instant settlement)

TARGET USERS

Who: Smallholder farmers (often unbanked) and regional crop traders/middlemen.

Where: Agricultural provinces in the Philippines (e.g., Nueva Ecija, Benguet).

Why they care: Farmers want guaranteed proof of what they are owed; honest traders want to build a reputation of trust to secure consistent suppliers.

CORE FEATURE (MVP)
Trader inputs "500kg Rice at 20 PHP/kg" into the web app → Farmer reviews on their phone and clicks "Sign/Approve" → Soroban contract executes, storing the agreement on-chain → Both screens instantly update with a verified "Transaction Receipt" ID.

WHY THIS WINS
This perfectly fits a hackathon because the MVP is highly focused and achievable within 48 hours. Judges will find it compelling because it uses blockchain not for speculation, but to solve a massive, real-world exploitation issue in the local Southeast Asian agricultural economy using Stellar's low-cost network.

OPTIONAL EDGE

Offline / low-connectivity support: Allow farmers to approve the transaction via a standard SMS text, which a local backend oracle then pushes to the Soroban smart contract.

📂 Project Structure

<img width="425" height="478" alt="image" src="https://github.com/user-attachments/assets/e83225b1-8da1-49e3-a476-8c9d767baf3a" />


Contract ID: CD7KVIAGEMJCOZAPW23IWQLGTKGZXY2QS3AN7WPAOO4PB7DQERTDD6BY

Compiler Version: Solidity ^0.8.20

Network: Ethereum Sepolia
<img width="1815" height="997" alt="Screenshot 2026-03-31 165608" src="https://github.com/user-attachments/assets/c7cf9bae-13ea-4145-af6d-c1c697ddbedd" />
<img width="1853" height="994" alt="Screenshot 2026-03-31 165638" src="https://github.com/user-attachments/assets/92efd37d-af9d-44b9-9491-0812c8465eac" />



