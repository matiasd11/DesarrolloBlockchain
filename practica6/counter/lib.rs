#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::prelude::*;
use ink::storage::Mapping;

#[ink::contract]
mod counter {
    use super::*;

    #[ink(storage)]
    pub struct Counter {
        owner: AccountId,
        counter: i32,
        whitelist: Mapping<AccountId, bool>,
    }

    #[ink(event)]
    pub struct CounterModified {
        #[ink(topic)]
        by: AccountId,
        new_value: i32,
    }

    #[ink(event)]
    pub struct WhitelistModified {
        #[ink(topic)]
        account: AccountId,
        added: bool,
    }

    impl Counter {
        /// Constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self {
                owner: caller,
                counter: 0,
                whitelist: Mapping::default(),
            }
        }

        fn ensure_owner(&self) {
            assert_eq!(self.env().caller(), self.owner, "Solo el propietario puede ejecutar esta función");
        }

        fn ensure_whitelisted(&self) {
            let caller = self.env().caller();
            assert!(
                self.whitelist.get(caller).unwrap_or(false),
                "No estás en la whitelist"
            );
        }

        fn _add_to_whitelist(&mut self, account: AccountId) {
            self.whitelist.insert(account, &true);
            self.env().emit_event(WhitelistModified {
                account,
                added: true,
            });
        }

        fn _remove_from_whitelist(&mut self, account: AccountId) {
            self.whitelist.insert(account, &false);
            self.env().emit_event(WhitelistModified {
                account,
                added: false,
            });
        }
        #[allow(clippy::arithmetic_side_effects)]
        fn _increment(&mut self) {
            self.counter =  self.counter + 1;
            self.env().emit_event(CounterModified {
                by: self.env().caller(),
                new_value: self.counter,
            });
        }
        #[allow(clippy::arithmetic_side_effects)]
        fn _decrement(&mut self) {
            self.counter = self.counter - 1;
            self.env().emit_event(CounterModified {
                by: self.env().caller(),
                new_value: self.counter,
            });
        }

        #[ink(message)]
        pub fn add_to_whitelist(&mut self, account: AccountId) {
            self.ensure_owner();
            self._add_to_whitelist(account);
        }

        #[ink(message)]
        pub fn remove_from_whitelist(&mut self, account: AccountId) {
            self.ensure_owner();
            self._remove_from_whitelist(account);
        }

        #[ink(message)]
        pub fn is_whitelisted(&self, account: AccountId) -> bool {
            self.whitelist.get(account).unwrap_or(false)
        }

        #[ink(message)]
        pub fn increment(&mut self) {
            self.ensure_whitelisted();
            self._increment();
        }

        #[ink(message)]
        pub fn decrement(&mut self) {
            self.ensure_whitelisted();
            self._decrement();
        }

        #[ink(message)]
        pub fn get_counter(&self) -> i32 {
            self.counter
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }
        
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env;

        #[ink::test]
        fn test_whitelist_and_counter() {
            let accounts = env::test::default_accounts::<env::DefaultEnvironment>();
            env::test::set_caller::<env::DefaultEnvironment>(accounts.alice);

            let mut contract = Counter::new();

            assert_eq!(contract.get_owner(), accounts.alice);

            contract.add_to_whitelist(accounts.alice);
            assert!(contract.is_whitelisted(accounts.alice));

            contract.increment();
            assert_eq!(contract.get_counter(), 1);

            contract.decrement();
            assert_eq!(contract.get_counter(), 0);

            contract.add_to_whitelist(accounts.bob);
            assert!(contract.is_whitelisted(accounts.bob));
            contract.remove_from_whitelist(accounts.bob);
            assert!(!contract.is_whitelisted(accounts.bob));
        }
    }
    
}
