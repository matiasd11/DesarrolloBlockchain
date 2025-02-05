#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::prelude::*;
use ink::storage::Mapping;

#[ink::contract]
mod wallet {
    use super::*;

    #[ink(storage)]
    pub struct Wallet {
        owner: AccountId,
        balance: Balance,
    }

    #[ink(event)]
    pub struct Deposit {
        #[ink(topic)]
        from: AccountId,
        amount: u128,
    }

    #[ink(event)]
    pub struct Withdrawal {
        #[ink(topic)]
        to: AccountId,
        amount: u128,
    }

    impl Wallet {
        /// Constructor del contrato
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            Self {
                owner: caller,
                balance: 0,
            }
        }

        /// Modificador para asegurar que solo el propietario pueda ejecutar la función
        fn only_owner(&self) {
            assert_eq!(
                self.env().caller(),
                self.owner,
                "Solo el propietario puede ejecutar esta función."
            );
        }

        /// Función privada para manejar el depósito
        fn _depositar(&mut self) {
            let amount = self.env().transferred_value();
            self.balance = self
                .balance
                .checked_add(amount)
                .expect("Desbordamiento al agregar el balance");
            self.env().emit_event(Deposit {
                from: self.env().caller(),
                amount,
            });
        }

        /// Función pública para realizar un depósito, llama a la función privada `_depositar`
        #[ink(message, payable)]
        pub fn depositar(&mut self) {
            self._depositar();
        }

        /// Función privada para manejar el retiro
        fn _retirar(&mut self, amount: Balance) {
            self.only_owner();

            assert!(
                self.balance >= amount,
                "Balance insuficiente para realizar el retiro"
            );

            self.balance = self
                .balance
                .checked_sub(amount)
                .expect("Desbordamiento al restar el balance");

            self.env()
                .transfer(self.owner, amount)
                .expect("La transferencia falló");

            self.env().emit_event(Withdrawal {
                to: self.owner,
                amount,
            });
        }

        /// Función pública para retirar fondos, solo puede ser llamada por el propietario
        #[ink(message)]
        pub fn retirar(&mut self, _amount: Balance) {
            self._retirar(_amount);
        }

        /// Función para obtener el balance actual del contrato
        #[ink(message)]
        pub fn get_balance(&self) -> Balance {
            self.balance
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env;

        #[ink::test]
        fn test_deposit() {
            let mut wallet = Wallet::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(1000000);

            wallet.depositar();
            assert_eq!(wallet.get_balance(), 1000000);
        }

        #[ink::test]
        fn test_withdraw() {
            let mut wallet = Wallet::new();
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            ink::env::test::set_value_transferred::<ink::env::DefaultEnvironment>(2000000);
            wallet.depositar();

            assert_eq!(wallet.get_balance(), 2000000);

            wallet.retirar(1000000);

            assert_eq!(wallet.get_balance(), 1000000);
        }

        #[ink::test]
        #[should_panic(expected = "Balance insuficiente para realizar el retiro")]
        fn test_withdraw_insufficient_funds() {
            let accounts = env::test::default_accounts::<env::DefaultEnvironment>();
            env::test::set_caller::<env::DefaultEnvironment>(accounts.alice);

            let mut contract = Wallet::new();

            env::test::set_account_balance::<env::DefaultEnvironment>(
                env::test::callee::<env::DefaultEnvironment>(),
                0,
            );

            let withdrawal_amount: Balance = 1000;
            contract.retirar(withdrawal_amount);
        }

        #[ink::test]
        #[should_panic(expected = "Solo el propietario puede ejecutar esta función.")]
        fn test_withdraw_not_owner() {
            let accounts = env::test::default_accounts::<env::DefaultEnvironment>();
            env::test::set_caller::<env::DefaultEnvironment>(accounts.alice);

            let mut contract = Wallet::new();

            let deposit_amount: Balance = 1000;
            env::test::set_value_transferred::<env::DefaultEnvironment>(deposit_amount);
            contract.depositar();

            env::test::set_caller::<env::DefaultEnvironment>(accounts.bob);
            let withdrawal_amount: Balance = 500;

            contract.retirar(withdrawal_amount);
        }
    }
}
