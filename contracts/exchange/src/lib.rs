#![no_std]

use soroban_sdk::{contract, contractimpl, token, Env, Address, IntoVal};

#[contract]
pub struct ExchangeContract;

#[contractimpl]
impl ExchangeContract {
    // In this function, i am doing a send, whereby users will select the token they want to send
    pub fn send(env: Env, sender: Address, receiver: Address, selected_token: Address, amount: i128) {
        //Now, i am checking for auth for the sender
        sender.require_auth_for_args((selected_token.clone(), amount).into_val(&env));

        let token = token::Client::new(&env, &selected_token);
        // Check sender balance of the specified token
        let balance = token.balance(&sender);
        if balance < amount {
            panic!("Not enough funds in your wallet");
        }

        // Transfer the token from sender to receiver
        token.transfer(&sender, &receiver, &amount);
    }

    // Exchange tokens: sender sends one token, and the receiver receives another
    pub fn exchange(
        env: Env,
        sender: Address,
        receiver: Address,
        send_token: Address,
        receive_token: Address,
        amount: i128,
    ) {
        sender.require_auth_for_args((send_token.clone(), receive_token.clone(), amount).into_val(&env));
        // receiver.require_auth_for_args((send_token.clone(), receive_token.clone(), amount).into_val(&env));

        let token_send = token::Client::new(&env, &send_token);
        let token_receive = token::Client::new(&env, &receive_token);

        let send_balance = token_send.balance(&sender);
        if send_balance < amount {
            panic!("Not enough balance to send");
        }

        // Check contract balance of the receive_token
        let receive_balance = token_receive.balance(&env.current_contract_address());
        if receive_balance < amount {
            panic!("Contract doesn't have enough tokens to send")
        }

        // Transfer the send_token from sender to contract
        token_send.transfer(&sender, &env.current_contract_address(), &amount);

        // Transfer the receive_token from contract to receiver
        token_receive.transfer(&env.current_contract_address(), &receiver, &amount);
        
    }

    // Read the contract address
    pub fn contract_address(env: Env) -> Address {
        env.current_contract_address()
    }

    // Read the balance of a specific token in the contract
    pub fn read_balance(env: Env, selected_token: Address, address: Address) -> i128 {
        let token = token::Client::new(&env, &selected_token);
        token.balance(&address)
    }
}

