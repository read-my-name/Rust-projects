#![no_std]
use soroban_sdk::{contractimpl, contract, symbol_short, Symbol, log, Env};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello() -> Symbol {
        symbol_short!("Hey")
    }

    pub fn hello_to(env:Env, name: Symbol){
        log!(&env,"World? {}!" ,name);
    }
}