# Decentralized Messaging

## Project Title

**Decentralized Messaging - Encrypted Peer-to-Peer Communication on Stellar Blockchain**

## Project Description

Decentralized Messaging is a blockchain-based encrypted peer-to-peer messaging application built on the Stellar network using Soroban smart contracts. The platform enables users to send and receive encrypted messages in a trustless, decentralized environment without relying on centralized servers or intermediaries. All messages are stored on-chain with end-to-end encryption, ensuring privacy, security, and immutability.

This smart contract provides the fundamental infrastructure for secure messaging, allowing users to:

- Send encrypted messages to any Stellar address
- Retrieve their messages securely with authentication
- Mark messages as read
- Maintain complete control over their communication data

## Project Vision

Our vision is to revolutionize digital communication by creating a truly decentralized, censorship-resistant messaging platform that prioritizes user privacy and data ownership. In an era where centralized messaging platforms control user data and can be subject to censorship or data breaches, we aim to provide an alternative that puts users back in control.

**Key Objectives:**

- **Privacy First**: All messages are encrypted, ensuring only sender and receiver can access content
- **Decentralization**: No single point of failure or control; messages stored on blockchain
- **User Ownership**: Users maintain complete ownership and control of their data
- **Censorship Resistance**: No central authority can block, delete, or modify messages
- **Transparency**: Open-source smart contracts that can be audited by anyone
- **Interoperability**: Built on Stellar, enabling cross-platform messaging capabilities

## Key Features

### 1. **Encrypted Message Sending**

- Users can send encrypted messages to any Stellar address
- Each message receives a unique ID for tracking and retrieval
- Automatic timestamping for message chronology
- Sender authentication required for security

### 2. **Secure Message Retrieval**

- Only authorized parties (sender or receiver) can access message content
- Built-in authentication mechanism prevents unauthorized access
- Efficient message lookup by message ID

### 3. **Read Receipt System**

- Receivers can mark messages as read
- Provides confirmation of message delivery and reading
- Only the intended receiver can update read status

### 4. **Message Counter**

- Public counter for total messages sent through the platform
- Enables statistics and analytics
- Demonstrates platform usage and adoption

### 5. **On-Chain Storage**

- All messages stored immutably on the Stellar blockchain
- Automatic Time-To-Live (TTL) extension for data persistence
- Guarantees message availability and integrity

### 6. **Authentication & Authorization**

- Stellar address-based authentication
- Granular access control for message operations
- Protection against impersonation and unauthorized access

### Current Prototype Scope

| **Layer**                        | **Description**                                      | **Status**                                   |
| -------------------------------- | ---------------------------------------------------- | -------------------------------------------- |
| **Smart Contract**               | Handles message creation, retrieval, and read status | ✅ Fully functional _(deployed on Testnet)_  |
| **Frontend (React)**             | Implements wallet connection, encryption, chat UI    | ✅ Functional                                |
| **Freighter Integration**        | Wallet connection and transaction signing            | ✅ Working                                   |
| **Soroban RPC Transaction Flow** | Submit, simulate, and query messages                 | Partially functional _(RPC updates ongoing)_ |

## Future Scope

### Short-term Enhancements (3-6 months)

1. **Message Deletion**: Allow users to delete their sent/received messages
2. **Bulk Message Retrieval**: Fetch multiple messages in a single query
3. **Message Filtering**: Filter messages by sender, date, or read status
4. **User Inbox Management**: Implement pagination for large message volumes
5. **Message Forwarding**: Enable users to forward messages to other addresses

### Medium-term Development (6-12 months)

1. **Group Messaging**: Support for encrypted group chats with multiple participants
2. **Message Attachments**: Enable sending of encrypted files and media
3. **Contact List Management**: Store and manage frequent contacts on-chain
4. **Message Threading**: Organize messages into conversation threads
5. **Notification System**: Event-based notifications for new messages
6. **Message Search**: Full-text search capabilities for encrypted messages
7. **Multi-device Support**: Sync messages across multiple devices

### Long-term Vision (1-2 years)

1. **Cross-Chain Messaging**: Enable messaging between different blockchain networks
2. **Decentralized Identity Integration**: Integrate with DID standards for enhanced privacy
3. **Voice and Video Calls**: Peer-to-peer encrypted audio/video communication
4. **Smart Contract Automation**: Scheduled messages and automated responses
5. **Reputation System**: Community-driven trust and spam prevention
6. **Decentralized Storage Integration**: Integrate IPFS or Arweave for large attachments
7. **Mobile and Desktop Applications**: Native apps with built-in wallet integration
8. **End-to-End Encryption Layers**: Additional encryption layers for enhanced security
9. **Message Expiration**: Self-destructing messages with time-based expiry
10. **Governance Token**: Community governance for platform development decisions

### Research & Innovation

- Zero-knowledge proof integration for enhanced privacy
- Quantum-resistant encryption algorithms
- AI-powered spam and malicious content detection (privacy-preserving)
- Integration with decentralized social media protocols
- Layer-2 scaling solutions for improved performance and reduced costs

---

## Getting Started

### Prerequisites

- Rust and Cargo installed
- Soroban CLI tools
- Stellar account with testnet/mainnet tokens

### Installation

```bash
# Clone the repository
git clone <repository-url>

# Build the contract
cargo build --target wasm32-unknown-unknown --release

# Deploy to Stellar testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/messaging_contract.wasm \
  --source <your-secret-key> \
  --network testnet
```

### Usage Example

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

## Contributing

We welcome contributions from the community! Please read our contributing guidelines and submit pull requests for any enhancements.

## License

This project is licensed under the MIT License.

## Contact

For questions, suggestions, or collaboration opportunities, please reach out to our development team.

---

**Built with on Stellar using Soroban**
