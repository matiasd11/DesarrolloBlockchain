#![cfg_attr(not(feature = "std"), no_std, no_main)]
use ink::prelude::{string::String};
use ink::storage::Mapping;
extern crate alloc;  

#[ink::contract]
mod voting_contract {
    use ink::prelude::{string::String};
    use ink::storage::Mapping;
    use alloc::vec::Vec;
    
    #[ink(storage)]
    pub struct VotingContract {
        candidates: Vec<String>,
        votes: Mapping<String, u32>,
        voters: Mapping<AccountId, bool>,
    }

    impl VotingContract {
        /// Crea una nueva instancia del contrato con una lista de candidatos proporcionada.
        #[ink(constructor)]
        pub fn new(candidates: Vec<String>) -> Self {
            let votes = Mapping::default();
            let voters = Mapping::default();

            VotingContract {
                candidates,
                votes,
                voters,
            }
        }
        /// Permite a un usuario emitir un voto por un candidato válido.
        #[ink(message)]
        pub fn vote(&mut self, candidate: String) {
            self.internal_vote(candidate);
        }
        /// Obtiene el número total de votos para un candidato específico.
        #[ink(message)]
        pub fn get_votes(&self, candidate: String) -> u32 {
            self.internal_get_votes(&candidate)
        }
         /// Declara el ganador de la votación basándose en el mayor número de votos.
        #[ink(message)]
        pub fn declare_winner(&self) -> Option<String> {
            self.internal_declare_winner()
        }
        
        fn internal_vote(&mut self, candidate: String) {
            let who = self.env().caller();
        
            if self.voters.contains(&who) {
                panic!("Ya has votado");
            }
        
            if !self.is_valid_candidate(&candidate) {
                panic!("Candidato no válido");
            }
        
            let current_votes = self.votes.get(&candidate).unwrap_or(0);
            let new_votes = current_votes.wrapping_add(1); // Use wrapping_add to handle the addition safely
            self.votes.insert(candidate.clone(), &new_votes); // Insert the updated vote count
            self.voters.insert(who, &true);
        }
        
        

        fn internal_get_votes(&self, candidate: &String) -> u32 {
            self.votes.get(candidate).unwrap_or(0)
        }

        fn internal_declare_winner(&self) -> Option<String> {
            let mut max_votes = 0;
            let mut winner = None;

            for candidate in &self.candidates {
                let votes = self.internal_get_votes(candidate);
                if votes > max_votes {
                    max_votes = votes;
                    winner = Some(candidate.clone());
                }
            }

            winner
        }

        fn is_valid_candidate(&self, candidate: &String) -> bool {
            self.candidates.contains(candidate)
        }
    }

    #[cfg(test)]
mod tests {
    use super::*;
    use ink::env;

    #[ink::test]
    fn test_vote() {
        let mut contract = VotingContract::new(vec!["Alice".to_string(), "Bob".to_string()]);

        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        contract.vote("Alice".to_string());
        assert_eq!(contract.get_votes("Alice".to_string()), 1);

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
        contract.vote("Bob".to_string());
        assert_eq!(contract.get_votes("Bob".to_string()), 1);

        assert_eq!(contract.get_votes("Alice".to_string()), 1);
        assert_eq!(contract.get_votes("Bob".to_string()), 1);
    }

    #[ink::test]
    fn test_declare_winner() {
        let mut contract = VotingContract::new(vec!["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()]);

        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        contract.vote("Alice".to_string());

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
        contract.vote("Bob".to_string());

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.charlie);
        contract.vote("Alice".to_string());

        let winner = contract.declare_winner();
        assert_eq!(winner, Some("Alice".to_string()));

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.django);
        contract.vote("Bob".to_string());

        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.eve);
        contract.vote("Bob".to_string());

        let new_winner = contract.declare_winner();
        assert_eq!(new_winner, Some("Bob".to_string()));
    }
}

}
