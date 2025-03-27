fn main() {
    println!("Hello, world!");
}
#![cfg_attr(not(feature = "std"), no_std)]

use ink::env::{DefaultEnvironment, Environment};

type AccountId = <DefaultEnvironment as Environment>::AccountId;

#[ink::contract]
mod nft_rental {
    use super::*;

    #[ink(storage)]
    pub struct NftRental {
        owner: AccountId,
    }

    impl NftRental {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                owner: Self::env().caller() 
            }
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }
    }
}