#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token, Address, BytesN, Env, Symbol,
};

fn setup_test(env: &Env) -> (BettingContractClient<'_>, Address, Address) {
    let contract_id = env.register(BettingContract, ());
    let client = BettingContractClient::new(env, &contract_id);
    let backend_signer = Address::generate(env);
    let bettor = Address::generate(env);
    client.initialize(&backend_signer);
    (client, backend_signer, bettor)
}

#[test]
fn test_place_bet_success() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, _backend_signer, bettor) = setup_test(&env);
    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_id = token_contract.address();
    let token_client = token::Client::new(&env, &token_id);
    let token_admin_client = token::StellarAssetClient::new(&env, &token_id);

    let amount = 1000i128;
    token_admin_client.mint(&bettor, &amount);

    let match_id = BytesN::from_array(&env, &[1u8; 32]);
    let bet_type = Symbol::new(&env, "win");
    let odds = 200; // 2.00

    let result = client.try_place_bet(&bettor, &token_id, &amount, &match_id, &bet_type, &odds);
    assert!(result.is_ok());

    // Verify funds were transferred
    assert_eq!(token_client.balance(&bettor), 0);
    assert_eq!(token_client.balance(&client.address), amount);
}

#[test]
fn test_prevent_double_betting() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, backend_signer, bettor) = setup_test(&env);
    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin);
    let token_id = token_contract.address();
    let token_admin_client = token::StellarAssetClient::new(&env, &token_id);

    let amount = 1000i128;
    token_admin_client.mint(&bettor, &(amount * 2));

    let match_id = BytesN::from_array(&env, &[1u8; 32]);
    let bet_type = Symbol::new(&env, "win");
    let odds = 200;

    // Enable double betting prevention
    client.set_prevent_double_betting(&backend_signer, &true);
    assert!(client.is_double_betting_prevented());

    // First bet
    client.place_bet(&bettor, &token_id, &amount, &match_id, &bet_type, &odds);

    // Second bet on same match by same user should fail
    let result = client.try_place_bet(&bettor, &token_id, &amount, &match_id, &bet_type, &odds);
    assert!(result.is_err());
}

#[test]
fn test_allow_double_betting_when_disabled() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (client, backend_signer, bettor) = setup_test(&env);
    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin);
    let token_id = token_contract.address();
    let token_admin_client = token::StellarAssetClient::new(&env, &token_id);

    let amount = 1000i128;
    token_admin_client.mint(&bettor, &(amount * 2));

    let match_id = BytesN::from_array(&env, &[1u8; 32]);
    let bet_type = Symbol::new(&env, "win");
    let odds = 200;

    // Ensure double betting is allowed (default)
    assert!(!client.is_double_betting_prevented());

    // First bet
    client.place_bet(&bettor, &token_id, &amount, &match_id, &bet_type, &odds);

    // Second bet on same match by same user should succeed
    let result = client.try_place_bet(&bettor, &token_id, &amount, &match_id, &bet_type, &odds);
    assert!(result.is_ok());
}

#[test]
fn executes_spin_once_per_spin_id() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _, executor) = setup_test(&env);

    let spin_id = BytesN::from_array(&env, &[1u8; 32]);
    let spin_hash = BytesN::from_array(&env, &[2u8; 32]);
    let signature = BytesN::from_array(&env, &[3u8; 64]);

    client.execute_spin(&spin_id, &spin_hash, &signature, &executor);
    assert_eq!(
        client.try_execute_spin(&spin_id, &spin_hash, &signature, &executor),
        Err(Ok(ContractError::DuplicateOperation))
    );
}

#[test]
fn rejects_replay_by_spin_hash() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _, executor) = setup_test(&env);

    let spin_hash = BytesN::from_array(&env, &[9u8; 32]);
    let signature = BytesN::from_array(&env, &[4u8; 64]);

    client.execute_spin(
        &BytesN::from_array(&env, &[7u8; 32]),
        &spin_hash,
        &signature,
        &executor,
    );

    assert_eq!(
        client.try_execute_spin(
            &BytesN::from_array(&env, &[8u8; 32]),
            &spin_hash,
            &signature,
            &executor,
        ),
        Err(Ok(ContractError::DuplicateOperation))
    );
}

#[test]
fn reports_spin_hash_usage() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _, executor) = setup_test(&env);

    let spin_id = BytesN::from_array(&env, &[10u8; 32]);
    let spin_hash = BytesN::from_array(&env, &[11u8; 32]);
    let signature = BytesN::from_array(&env, &[5u8; 64]);

    assert!(!client.is_spin_hash_used(&spin_hash));
    client.execute_spin(&spin_id, &spin_hash, &signature, &executor);
    assert!(client.is_spin_hash_used(&spin_hash));
}

#[test]
fn supports_ttl_cleanup_for_spin_hashes() {
    let env = Env::default();
    env.mock_all_auths();

    let (client, _, executor) = setup_test(&env);

    let spin_id = BytesN::from_array(&env, &[12u8; 32]);
    let spin_hash = BytesN::from_array(&env, &[13u8; 32]);
    let signature = BytesN::from_array(&env, &[6u8; 64]);

    client.execute_spin_with_ttl(&spin_id, &spin_hash, &signature, &executor, &Some(5));
    assert!(client.is_spin_hash_used(&spin_hash));

    env.ledger().with_mut(|li| {
        li.timestamp += 6;
    });

    assert!(client.cleanup_spin_hash(&spin_hash));
    assert!(!client.is_spin_hash_used(&spin_hash));
}
