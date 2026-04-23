#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, log, symbol_short, Address, Env, String, Symbol,
};

// Message structure to store encrypted messages
#[contracttype]
#[derive(Clone)]
pub struct Message {
    pub msg_id: u64,
    pub sender: Address,
    pub receiver: Address,
    pub encrypted_content: String,
    pub timestamp: u64,
    pub is_read: bool,
}

// Mapping message ID to Message
#[contracttype]
pub enum MessageBook {
    Message(u64),
}

// Counter for generating unique message IDs
const MSG_COUNT: Symbol = symbol_short!("MSG_CNT");

// Mapping user address to their message IDs (inbox)
#[contracttype]
pub enum UserInbox {
    Inbox(Address),
}

#[contract]
pub struct MessagingContract;

#[contractimpl]
impl MessagingContract {
    // Function to send an encrypted message
    pub fn send_message(
        env: Env,
        sender: Address,
        receiver: Address,
        encrypted_content: String,
    ) -> u64 {
        sender.require_auth();

        let mut msg_count: u64 = env.storage().instance().get(&MSG_COUNT).unwrap_or(0);
        msg_count += 1;

        let timestamp = env.ledger().timestamp();

        let message = Message {
            msg_id: msg_count,
            sender: sender.clone(),
            receiver: receiver.clone(),
            encrypted_content,
            timestamp,
            is_read: false,
        };

        env.storage()
            .instance()
            .set(&MessageBook::Message(msg_count), &message);
        env.storage().instance().set(&MSG_COUNT, &msg_count);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Message sent with ID: {}", msg_count);

        msg_count
    }

    // Function to retrieve a specific message
    pub fn get_message(env: Env, msg_id: u64, user: Address) -> Message {
        user.require_auth();

        let message: Message = env
            .storage()
            .instance()
            .get(&MessageBook::Message(msg_id))
            .unwrap_or(Message {
                msg_id: 0,
                sender: user.clone(),
                receiver: user.clone(),
                encrypted_content: String::from_str(&env, "Not_Found"),
                timestamp: 0,
                is_read: false,
            });

        if message.sender == user || message.receiver == user {
            message
        } else {
            panic!("Unauthorized access to message");
        }
    }

    // Function to mark message as read
    pub fn mark_as_read(env: Env, msg_id: u64, receiver: Address) {
        receiver.require_auth();

        let mut message: Message = env
            .storage()
            .instance()
            .get(&MessageBook::Message(msg_id))
            .unwrap_or_else(|| panic!("Message not found"));

        if message.receiver != receiver {
            panic!("Only receiver can mark message as read");
        }

        message.is_read = true;

        env.storage()
            .instance()
            .set(&MessageBook::Message(msg_id), &message);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Message {} marked as read", msg_id);
    }

    // Function to get total message count
    pub fn get_message_count(env: Env) -> u64 {
        env.storage().instance().get(&MSG_COUNT).unwrap_or(0)
    }
}
