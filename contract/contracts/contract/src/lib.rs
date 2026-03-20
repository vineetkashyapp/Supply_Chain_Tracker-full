#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, Address, Env, String,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotFound = 1,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Product(String),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Product {
    pub origin: String,
    pub status: String,
    pub owner: Address,
}

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    /// Anyone can create a product. Caller provides id, origin, status, and initial owner.
    pub fn create_product(env: Env, id: String, origin: String, status: String, owner: Address) {
        owner.require_auth();
        let product = Product {
            origin,
            status,
            owner,
        };
        env.storage()
            .persistent()
            .set(&DataKey::Product(id), &product);
    }

    /// Anyone can update any product's status (permissionless).
    pub fn update_status(env: Env, caller: Address, id: String, new_status: String) {
        caller.require_auth();
        let mut product: Product = env
            .storage()
            .persistent()
            .get(&DataKey::Product(id.clone()))
            .unwrap_or_else(|| panic_with_error!(&env, Error::NotFound));
        product.status = new_status;
        env.storage()
            .persistent()
            .set(&DataKey::Product(id), &product);
    }

    /// Anyone can transfer ownership of any product (permissionless).
    pub fn transfer_ownership(env: Env, caller: Address, id: String) {
        caller.require_auth();
        let mut product: Product = env
            .storage()
            .persistent()
            .get(&DataKey::Product(id.clone()))
            .unwrap_or_else(|| panic_with_error!(&env, Error::NotFound));
        product.owner = caller;
        env.storage()
            .persistent()
            .set(&DataKey::Product(id), &product);
    }

    /// View product details.
    pub fn get_product(env: Env, id: String) -> Product {
        env.storage()
            .persistent()
            .get(&DataKey::Product(id))
            .unwrap_or_else(|| panic_with_error!(&env, Error::NotFound))
    }
}

mod test;
