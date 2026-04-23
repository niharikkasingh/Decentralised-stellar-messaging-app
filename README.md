# Decentralized Messaging - Encrypted Peer-to-Peer Communication on Stellar Blockchain
## 🔗 Overview
Decentralized Messaging is a blockchain-based encrypted peer-to-peer messaging application built on the Stellar network using Soroban smart contracts. The system enables users to send and receive encrypted messages in a trustless environment without relying on centralized servers.
All messages are stored on-chain with end-to-end encryption, ensuring privacy, security, and immutability.
---
## 🏗️ Architecture
### Technology Stack
- **Blockchain**: Stellar Testnet with Soroban Smart Contracts  
- **Smart Contract**: Rust (Soroban)  
- **Frontend**: React  
- **Wallet Integration**: Freighter Wallet  
- **Transaction Flow**: Soroban RPC  
---
## ⚙️ System Components
### 1. Smart Contract (Soroban)
- Handles message creation, retrieval, and read status  
- Stores encrypted messages on-chain  
- Maintains message IDs and timestamps  
### 2. Frontend Application
- User interface for sending and receiving messages  
- Freighter wallet integration for authentication  
- Encryption handling and message display  
### 3. Blockchain Layer
- Stores messages immutably on Stellar  
- Ensures decentralization and data integrity  
---
## 📁 Project Structure
```
messaging/
├── contract/        # Soroban smart contract
├── frontend/        # React frontend
└── README.md
```
---
## 🚀 Getting Started
### Prerequisites
- Rust and Cargo  
- Soroban CLI  
- Stellar account (testnet/mainnet)  
- Freighter Wallet  
---
## Installation
```bash
# Clone the repository
git clone <repository-url>
# Build the contract
cargo build --target wasm32-unknown-unknown --release
```
---
## Deployment
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/messaging_contract.wasm \
  --source <your-secret-key> \
  --network testnet
```
---
## Usage Example
```bash
# Send a message
soroban contract invoke \
  --id <contract-id> \
  --source <sender-secret-key> \
  --network testnet \
  -- send_message \
  --sender <sender-address> \
  --receiver <receiver-address> \
  --encrypted_content "encrypted_message_here"
```
---
## Current Scope
| Layer | Description | Status |
|------|------------|--------|
| Smart Contract | Handles message creation, retrieval, and read status | Functional (deployed on Testnet) |
| Frontend (React) | Wallet connection, encryption, and chat interface | Functional |
| Freighter Integration | Wallet authentication and transaction signing | Functional |
| Soroban RPC Transaction Flow | Message submission, simulation, and querying | Partially functional |
## Features
* Encrypted peer-to-peer messaging between Stellar addresses
* On-chain message storage ensuring immutability
* Secure message retrieval with authentication
* Read receipt system for tracking message status
* Unique message IDs and timestamping
* Decentralized architecture with no central authority

---
## Security Features
* End-to-end encryption of messages
* Address-based authentication
* Protection against unauthorized access
* Immutable on-chain data storage
---
## Future Scope
* Message deletion and filtering
* Bulk message retrieval
* Group messaging support
* File and media attachments
* Message search and threading
* Notification system
* Cross-device synchronization

---
## License

This project is licensed under the MIT License.

## Contact

For questions, suggestions, or collaboration opportunities, please reach out to our development team.

---

**Built with on Stellar using Soroban**
