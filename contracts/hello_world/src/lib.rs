#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, BytesN, symbol_short};

// Struct for storing identity verification status
#[contracttype]
#[derive(Clone)]
pub struct IdentityStatus {
    pub verified: bool,       // True if the identity is verified
    pub verification_time: u64 // Timestamp of when the identity was verified
}

// Symbol to represent all identity statuses
const ALL_IDENTITIES: Symbol = symbol_short!("ALL_IDEN");

// Mapping unique identity ID to IdentityStatus
#[contracttype]
pub enum IdentityBook {
    Identity(BytesN<32>)  // Key for each identity is a 32-byte identifier (e.g., hash of user data)
}

// Contract for the decentralized identity verification system
#[contract]
pub struct IdentityVerificationContract;

#[contractimpl]
impl IdentityVerificationContract {
    // Register a new identity
    pub fn register_identity(env: Env, identity_id: BytesN<32>) -> bool {
        // Check if the identity already exists
        let identity_status = Self::get_identity_status(env.clone(), identity_id.clone());

        if identity_status.verified {
            log!(&env, "Identity already registered and verified");
            return false;
        }

        // Set up the initial verification status
        let new_status = IdentityStatus {
            verified: false,
            verification_time: 0,
        };

        // Store the identity status in the contract storage
        env.storage().instance().set(&IdentityBook::Identity(identity_id.clone()), &new_status);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Identity registered: {:?}", identity_id);
        true
    }

    // Verify an identity
    pub fn verify_identity(env: Env, identity_id: BytesN<32>) -> bool {
        let mut identity_status = Self::get_identity_status(env.clone(), identity_id.clone());

        // If the identity is already verified, return early
        if identity_status.verified {
            log!(&env, "Identity already verified: {:?}", identity_id);
            return false;
        }

        // Update the verification status and timestamp
        let timestamp = env.ledger().timestamp();
        identity_status.verified = true;
        identity_status.verification_time = timestamp;

        // Store the updated identity status in the contract storage
        env.storage().instance().set(&IdentityBook::Identity(identity_id.clone()), &identity_status);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Identity verified: {:?}", identity_id);
        true
    }

    // Get the verification status of an identity
    pub fn get_identity_status(env: Env, identity_id: BytesN<32>) -> IdentityStatus {
        // Retrieve the identity status from storage
        env.storage()
            .instance()
            .get(&IdentityBook::Identity(identity_id.clone()))
            .unwrap_or(IdentityStatus {
                verified: false,
                verification_time: 0,
            })
    }

    // Check if an identity is verified
    pub fn is_verified(env: Env, identity_id: BytesN<32>) -> bool {
        let identity_status = Self::get_identity_status(env, identity_id);
        identity_status.verified
    }
}
