#![cfg(test)]

use super::*;
use soroban_sdk::{Env, U256};

#[test]
fn test_placeholder() {
    let env = Env::default();
    let _contract_id = env.register(SettlementContract, ());
    // Tests will be implemented here
}

#[test]
fn test_settle_win_loss_draw() {
    use soroban_sdk::{testutils::Address as _, Address};

    let env = Env::default();
    env.mock_all_auths();

    let backend = Address::generate(&env);

    // Deploy balance ledger and initialize
    let bl_contract_id = env.register(balance_ledger::BalanceLedgerContract, ());
    let bl_client = balance_ledger::BalanceLedgerContractClient::new(&env, &bl_contract_id);
    bl_client.initialize(&backend);

    // Deploy settlement contract and initialize with balance ledger address
    let st_contract_id = env.register(SettlementContract, ());
    let st_client = SettlementContractClient::new(&env, &st_contract_id);
    let bl_addr = Address::Contract(bl_contract_id.clone());
    st_client.initialize(&backend, &bl_addr);

    // Prepare bettor and winner
    let bettor = Address::generate(&env);
    let winner = Address::generate(&env);

    // Fund and lock bettor funds
    bl_client.set_balance(&bettor, &1_000, &0);
    bl_client.lock_funds(&bettor, &100);

    let bet_id = U256::from_u64(42);

    // Settle WIN: bettor locked 100 -> winner gets payout 200
    let win_sym = soroban_sdk::Symbol::short("WIN");
    let res = st_client.settle_bet(&bet_id, &bettor, &Some(winner.clone()), &100, &200, &win_sym);
    assert!(res.is_ok());

    // Check balances: bettor locked decreased by 100, winner withdrawable increased by 200
    let bettor_balance = bl_client.get_balance(&bettor);
    assert_eq!(bettor_balance.locked, 0);

    let winner_balance = bl_client.get_withdrawable(&winner);
    assert_eq!(winner_balance, 200);

    // Attempt to re-settle same bet -> should fail
    let res2 = st_client.try_settle_bet(&bet_id, &bettor, &Some(winner.clone()), &100, &200, &win_sym);
    assert!(res2.is_err());

    // Test DRAW / refund for another bet
    let bet_id2 = U256::from_u64(43);
    bl_client.set_balance(&bettor, &500, &0);
    bl_client.lock_funds(&bettor, &50);
    let draw_sym = soroban_sdk::Symbol::short("DRAW");
    let res3 = st_client.settle_bet(&bet_id2, &bettor, &None, &50, &0, &draw_sym);
    assert!(res3.is_ok());

    let after_refund = bl_client.get_balance(&bettor);
    assert_eq!(after_refund.withdrawable, 500);
    assert_eq!(after_refund.locked, 0);
}
