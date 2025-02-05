#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use self::user_registry::UserRegistryRef;

#[ink::contract]
mod user_registry {
    use ink::storage::Mapping;
    use ink::prelude::string::String;
    
    #[ink(storage)]
    pub struct UserRegistry {
        owner: AccountId,
        users: Mapping<AccountId, String>,
    }

    #[ink(event)]
    pub struct UserRegistered {
        #[ink(topic)]
        account: AccountId,
        username: String,
    }

    impl UserRegistry {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                users: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn register_user(&mut self, username: String) {
            self.ensure_user_not_registered(Self::env().caller());
            self.add_user(Self::env().caller(), username);
        }

        #[ink(message)]
        pub fn get_username(&self, account: AccountId) -> Option<String> {
            self.find_username(account)
        }

        #[ink(message)]
        pub fn is_user_registered(&self, account: AccountId) -> bool {
            self.user_exists(account)
        }

        /// Verifica si un usuario ya está registrado
        fn user_exists(&self, account: AccountId) -> bool {
            self.users.contains(account)
        }

        /// Asegura que un usuario no esté registrado
        fn ensure_user_not_registered(&self, account: AccountId) {
            assert!(!self.user_exists(account), "El usuario ya está registrado");
        }

        /// Agrega un usuario al registro
        fn add_user(&mut self, account: AccountId, username: String) {
            self.users.insert(account, &username);
        }

        /// Obtiene el nombre de usuario asociado a una cuenta
        fn find_username(&self, account: AccountId) -> Option<String> {
            self.users.get(account)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test;

        #[ink::test]
        fn register_user_works() {
            let mut registry = UserRegistry::new();
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();

            let username = String::from("Alice");
            registry.register_user(username.clone());
            assert_eq!(registry.get_username(accounts.alice), Some(username));
        }

        #[ink::test]
        #[should_panic(expected = "El usuario ya está registrado")]
        fn register_user_twice_should_fail() {
            let mut registry = UserRegistry::new();
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();

            let username = String::from("Alice");
            registry.register_user(username.clone());
            registry.register_user(username.clone());
        }

        #[ink::test]
        fn is_user_registered_works() {
            let mut registry = UserRegistry::new();
            let accounts = test::default_accounts::<ink::env::DefaultEnvironment>();

            registry.register_user(String::from("Alice"));
            assert!(registry.is_user_registered(accounts.alice));
            assert!(!registry.is_user_registered(accounts.bob));
        }
    }
}
