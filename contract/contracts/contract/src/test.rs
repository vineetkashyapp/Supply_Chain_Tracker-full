#![cfg(test)]
use super::*;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Env, String};

#[test]
fn test_create_product() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let id = String::from_str(&env, "PROD-001");
    client.create_product(
        &id,
        &String::from_str(&env, "Factory A"),
        &String::from_str(&env, "Created"),
        &owner,
    );

    let product = client.get_product(&id);
    assert_eq!(product.origin, String::from_str(&env, "Factory A"));
    assert_eq!(product.status, String::from_str(&env, "Created"));
    assert_eq!(product.owner, owner);
}

#[test]
fn test_update_status() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let id = String::from_str(&env, "PROD-001");
    client.create_product(
        &id,
        &String::from_str(&env, "Factory A"),
        &String::from_str(&env, "Created"),
        &owner,
    );

    // Anyone can update status (permissionless)
    let updater = Address::generate(&env);
    client.update_status(&updater, &id, &String::from_str(&env, "In Transit"));

    let product = client.get_product(&id);
    assert_eq!(product.status, String::from_str(&env, "In Transit"));
    assert_eq!(product.owner, owner); // owner unchanged
}

#[test]
fn test_transfer_ownership() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let id = String::from_str(&env, "PROD-001");
    client.create_product(
        &id,
        &String::from_str(&env, "Factory A"),
        &String::from_str(&env, "Shipped"),
        &owner,
    );

    let new_owner = Address::generate(&env);
    client.transfer_ownership(&new_owner, &id);

    let product = client.get_product(&id);
    assert_eq!(product.owner, new_owner);
}

#[test]
fn test_multiple_products() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    client.create_product(
        &String::from_str(&env, "P1"),
        &String::from_str(&env, "China"),
        &String::from_str(&env, "Manufactured"),
        &owner,
    );
    client.create_product(
        &String::from_str(&env, "P2"),
        &String::from_str(&env, "USA"),
        &String::from_str(&env, "Assembled"),
        &owner,
    );

    let p1 = client.get_product(&String::from_str(&env, "P1"));
    assert_eq!(p1.origin, String::from_str(&env, "China"));
    let p2 = client.get_product(&String::from_str(&env, "P2"));
    assert_eq!(p2.origin, String::from_str(&env, "USA"));
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")]
fn test_product_not_found() {
    let env = Env::default();
    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);
    client.get_product(&String::from_str(&env, "NONEXISTENT"));
}
