#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod message_wall {
    use ink::prelude::{string::String, vec::Vec};
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct MessageWall {
        owner: AccountId,
        user_registry_address: AccountId,
        messages: Mapping<AccountId, Vec<String>>,
        #[cfg(test)]
        test_users: Mapping<AccountId, bool>,  // Simulación para pruebas
    }

    impl MessageWall {
        #[ink(constructor)]
        pub fn new(user_registry_address: AccountId) -> Self {
            Self::init(user_registry_address)
        }

        #[cfg(test)]
        #[ink(constructor)]
        pub fn new_mock() -> Self {
            let mut instance = Self::init(AccountId::from([0x0; 32]));
            instance.test_users.insert(instance.owner, &true);
            instance
        }

        fn init(user_registry_address: AccountId) -> Self {
            Self {
                owner: Self::env().caller(),
                user_registry_address,
                messages: Mapping::default(),
                #[cfg(test)]
                test_users: Mapping::default(),
            }
        }

        #[ink(message)]
        pub fn post_message(&mut self, content: String) {
            self._post_message(content);
        }

        fn _post_message(&mut self, content: String) {
            let caller = Self::env().caller();
            assert!(
                self.is_user_registered(caller),
                "Usuario no registrado."
            );
            let mut user_messages = self.messages.get(caller).unwrap_or_default();
            user_messages.push(content);
            self.messages.insert(caller, &user_messages);
        }

        #[ink(message)]
        pub fn get_messages(&self, user: AccountId) -> Vec<String> {
            self._get_messages(user)
        }

        fn _get_messages(&self, user: AccountId) -> Vec<String> {
            self.messages.get(user).unwrap_or_default()
        }

        fn is_user_registered(&self, account: AccountId) -> bool {
            #[cfg(not(test))]
            {
                ink::env::call::build_call::<ink::env::DefaultEnvironment>()
                    .call(self.user_registry_address)
                    .exec_input(
                        ink::env::call::ExecutionInput::new(ink::env::call::Selector::new([
                            0x33, 0x44, 0x55, 0x66, // Selector de `is_user_registered`
                        ]))
                        .push_arg(account),
                    )
                    .returns::<bool>()
                    .invoke()
            }

            #[cfg(test)]
            {
                self.test_users.get(account).unwrap_or(false)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_post_message() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut message_wall = MessageWall::new_mock();

            message_wall.test_users.insert(accounts.alice, &true);

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            message_wall.post_message("Hola mundo!".to_string());

            let messages = message_wall.get_messages(accounts.alice);
            assert_eq!(messages.len(), 1);
            assert_eq!(messages[0], "Hola mundo!");
        }

        #[ink::test]
        #[should_panic(expected = "Usuario no registrado.")]
        fn test_unregistered_user_cannot_post() {
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            let mut message_wall = MessageWall::new_mock();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
            message_wall.post_message("Mensaje no permitido".to_string());
        }
    }
}
