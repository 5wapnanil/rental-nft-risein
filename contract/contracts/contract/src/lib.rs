#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address};

#[contract]
pub struct RentalNFT;

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Owner(u32),
    Renter(u32),
    Expiry(u32),
}

#[contractimpl]
impl RentalNFT {

    pub fn mint(env: Env, to: Address, token_id: u32) {
        to.require_auth();
        env.storage().instance().set(&DataKey::Owner(token_id), &to);
    }

    pub fn rent(env: Env, token_id: u32, renter: Address, duration: u64) {
        renter.require_auth();

        let owner: Address = env
            .storage()
            .instance()
            .get(&DataKey::Owner(token_id))
            .expect("NFT not minted");

        let current_time = env.ledger().timestamp();
        let expiry = current_time + duration;

        env.storage().instance().set(&DataKey::Renter(token_id), &renter);
        env.storage().instance().set(&DataKey::Expiry(token_id), &expiry);
    }

    pub fn get_user(env: Env, token_id: u32) -> Address {
        let current_time = env.ledger().timestamp();

        if let Some(expiry) = env.storage().instance().get::<_, u64>(&DataKey::Expiry(token_id)) {
            if current_time < expiry {
                return env
                    .storage()
                    .instance()
                    .get(&DataKey::Renter(token_id))
                    .unwrap();
            }
        }

        env.storage()
            .instance()
            .get(&DataKey::Owner(token_id))
            .unwrap()
    }

    pub fn transfer(env: Env, from: Address, to: Address, token_id: u32) {
        from.require_auth();

        let owner: Address = env
            .storage()
            .instance()
            .get(&DataKey::Owner(token_id))
            .expect("NFT not found");

        if owner != from {
            panic!("Not owner");
        }

        env.storage().instance().set(&DataKey::Owner(token_id), &to);
    }
}