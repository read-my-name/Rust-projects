#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short, Address};

#[contract]
pub struct Contract;

const STORAGE: Symbol = symbol_short!("STORAGE");

#[contractimpl]
impl Contract {
    pub fn save(env: Env, user: Address ,number: u32) -> u32{
        env.storage().instance().set(&user, &number);
        number  
    }

    pub fn load(env: Env, user:Address) ->u32{
              env.storage().instance().get(&user).unwrap()
    }
}