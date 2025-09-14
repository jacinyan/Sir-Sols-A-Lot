use std::collections::HashMap;
use solana_sdk::signature::Keypair;
use solana_sdk::pubkey::Pubkey;


//#region
// use solana_sdk::{
//     signature::{Keypair, Signer},
//     pubkey::Pubkey,
// };
//
// /// Represents a Solana wallet with its keypair and public key.
// pub struct Wallet {
//     pub keypair: Keypair,
//     pub pubkey: Pubkey,
// }
//
// impl Wallet {
//     /// Creates a new random Solana wallet.
//     pub fn new_random() -> Self {
//         let keypair = Keypair::new();
//         let pubkey = keypair.pubkey();
//         Wallet { keypair, pubkey }
//     }
//
//     /// Creates a Solana wallet from an existing keypair.
//     pub fn from_keypair(keypair: Keypair) -> Self {
//         let pubkey = keypair.pubkey();
//         Wallet { keypair, pubkey }
//     }
//
//     /// Returns the public key of the wallet.
//     pub fn get_pubkey(&self) -> &Pubkey {
//         &self.pubkey
//     }
//
//     /// Returns a reference to the keypair of the wallet.
//     pub fn get_keypair(&self) -> &Keypair {
//         &self.keypair
//     }
// }
//
// // Example usage:
// fn main() {
//     let wallet = Wallet::new_random();
//     println!("Wallet Public Key: {}", wallet.get_pubkey());
//
//     // You can also create a wallet from a specific keypair if you have one
//     // let existing_keypair = Keypair::from_bytes(&[/* your 64-byte secret key here */]).unwrap();
//     // let wallet_from_existing = Wallet::from_keypair(existing_keypair);
//     // println!("Wallet from existing keypair Public Key: {}", wallet_from_existing.get_pubkey());
// }
//#endregion


pub struct WalletManager {
    wallets: HashMap<String, Keypair>,
    counter: usize,
}

impl WalletManager {
    pub fn new() -> Self {
        Self {
            wallets: HashMap::new(),
            counter: 0,
        }
    }

    pub fn generate_wallet(&mut self, _name: String) -> Result<Pubkey, String> {
        todo!("wallet generation functionality pending")
    }

    pub fn import_wallet(&mut self, _name: String, _private_key_bytes: &[u8]) -> Result<Pubkey, String> {
        unimplemented!("wallet import from private key")
    }

    pub fn get_wallet(&self, _name: &str) -> Result<&Keypair, String> {
        todo!("wallet retrieval by name")
    }

    pub fn get_pubkey(&self, _name: &str) -> Result<Pubkey, String> {
        unimplemented!("public key extraction")
    }

    pub fn list_wallets(&self) -> Vec<(String, Pubkey)> {
        todo!("enumerate all managed wallets")
    }

    pub fn remove_wallet(&mut self, _name: &str) -> Result<(), String> {
        unimplemented!("wallet removal operation")
    }

    pub fn wallet_count(&self) -> usize {
        todo!("count total managed wallets")
    }

    // Factory methods for batch operations
    pub fn batch_generate(&mut self, _prefix: &str, _count: usize) -> Result<Vec<String>, String> {
        unimplemented!("batch wallet generation")
    }

    pub fn batch_generate_auto_named(&mut self, _count: usize) -> Result<Vec<String>, String> {
        todo!("auto-named batch generation")
    }

    pub fn create_task_group(&mut self, _task_name: &str, _count: usize) -> Result<TaskGroup, String> {
        unimplemented!("task group creation")
    }

    pub fn get_task_group(&self, _task_name: &str) -> Result<Vec<String>, String> {
        todo!("retrieve task group wallets")
    }

    pub fn batch_get_pubkeys(&self, _wallet_names: &[String]) -> Result<Vec<(String, Pubkey)>, String> {
        unimplemented!("batch pubkey retrieval")
    }

    pub fn get_available_wallets(&self, _limit: Option<usize>) -> Vec<String> {
        todo!("get unassigned wallets")
    }

    pub fn assign_to_task(&mut self, _wallet_names: &[String], _task_name: &str) -> Result<(), String> {
        unimplemented!("assign wallets to task")
    }

    pub fn release_from_task(&mut self, _task_name: &str) -> Result<Vec<String>, String> {
        todo!("release wallets from task")
    }
}

#[derive(Debug, Clone)]
pub struct TaskGroup {
    pub task_name: String,
    pub wallet_names: Vec<String>,
    pub created_at: std::time::SystemTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_wallet_manager() {
        let manager = WalletManager::new();
        assert_eq!(manager.wallet_count(), 0);
    }

    #[test]
    fn test_generate_new_wallet() {
        let mut manager = WalletManager::new();

        let result = manager.generate_wallet("test_wallet".to_string());
        assert!(result.is_ok());

        let pubkey = result.unwrap();
        assert_eq!(pubkey.to_bytes().len(), 32);
        assert_eq!(manager.wallet_count(), 1);
    }

    #[test]
    fn test_generate_duplicate_wallet_fails() {
        let mut manager = WalletManager::new();

        let first_result = manager.generate_wallet("test_wallet".to_string());
        assert!(first_result.is_ok());

        let second_result = manager.generate_wallet("test_wallet".to_string());
        assert!(second_result.is_err());
        assert_eq!(manager.wallet_count(), 1);
    }

    #[test]
    fn test_get_wallet_pubkey() {
        let mut manager = WalletManager::new();

        let pubkey = manager.generate_wallet("test_wallet".to_string()).unwrap();
        let retrieved_pubkey = manager.get_pubkey("test_wallet").unwrap();

        assert_eq!(pubkey, retrieved_pubkey);
    }

    #[test]
    fn test_get_nonexistent_wallet_fails() {
        let manager = WalletManager::new();

        let result = manager.get_pubkey("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_wallets() {
        let mut manager = WalletManager::new();

        let pubkey1 = manager.generate_wallet("wallet1".to_string()).unwrap();
        let pubkey2 = manager.generate_wallet("wallet2".to_string()).unwrap();

        let wallets = manager.list_wallets();
        assert_eq!(wallets.len(), 2);

        let wallet_names: Vec<&String> = wallets.iter().map(|(name, _)| name).collect();
        assert!(wallet_names.contains(&&"wallet1".to_string()));
        assert!(wallet_names.contains(&&"wallet2".to_string()));

        let pubkeys: Vec<&Pubkey> = wallets.iter().map(|(_, pubkey)| pubkey).collect();
        assert!(pubkeys.contains(&&pubkey1));
        assert!(pubkeys.contains(&&pubkey2));
    }

    #[test]
    fn test_remove_wallet() {
        let mut manager = WalletManager::new();

        manager.generate_wallet("test_wallet".to_string()).unwrap();
        assert_eq!(manager.wallet_count(), 1);

        let result = manager.remove_wallet("test_wallet");
        assert!(result.is_ok());
        assert_eq!(manager.wallet_count(), 0);

        let get_result = manager.get_pubkey("test_wallet");
        assert!(get_result.is_err());
    }

    #[test]
    fn test_remove_nonexistent_wallet_fails() {
        let mut manager = WalletManager::new();

        let result = manager.remove_wallet("nonexistent");
        assert!(result.is_err());
    }

    // Factory method tests
    #[test]
    fn test_batch_generate_wallets() {
        let mut manager = WalletManager::new();

        let result = manager.batch_generate("soldier", 5);
        assert!(result.is_ok());

        let wallet_names = result.unwrap();
        assert_eq!(wallet_names.len(), 5);
        assert_eq!(manager.wallet_count(), 5);

        // Check naming convention
        assert!(wallet_names.contains(&"soldier_0".to_string()));
        assert!(wallet_names.contains(&"soldier_4".to_string()));
    }

    #[test]
    fn test_batch_generate_auto_named() {
        let mut manager = WalletManager::new();

        let result = manager.batch_generate_auto_named(3);
        assert!(result.is_ok());

        let wallet_names = result.unwrap();
        assert_eq!(wallet_names.len(), 3);
        assert_eq!(manager.wallet_count(), 3);

        // Should use counter for auto-naming
        assert!(wallet_names.contains(&"wallet_0".to_string()));
        assert!(wallet_names.contains(&"wallet_1".to_string()));
        assert!(wallet_names.contains(&"wallet_2".to_string()));
    }

    #[test]
    fn test_create_task_group() {
        let mut manager = WalletManager::new();

        let result = manager.create_task_group("kamino_farming", 3);
        assert!(result.is_ok());

        let task_group = result.unwrap();
        assert_eq!(task_group.task_name, "kamino_farming");
        assert_eq!(task_group.wallet_names.len(), 3);
        assert_eq!(manager.wallet_count(), 3);
    }

    #[test]
    fn test_get_task_group() {
        let mut manager = WalletManager::new();

        manager.create_task_group("test_task", 2).unwrap();
        let result = manager.get_task_group("test_task");

        assert!(result.is_ok());
        let wallets = result.unwrap();
        assert_eq!(wallets.len(), 2);
    }

    #[test]
    fn test_get_nonexistent_task_group() {
        let manager = WalletManager::new();

        let result = manager.get_task_group("nonexistent_task");
        assert!(result.is_err());
    }

    #[test]
    fn test_batch_get_pubkeys() {
        let mut manager = WalletManager::new();

        let wallet_names = manager.batch_generate("test", 3).unwrap();
        let result = manager.batch_get_pubkeys(&wallet_names);

        assert!(result.is_ok());
        let pubkey_pairs = result.unwrap();
        assert_eq!(pubkey_pairs.len(), 3);

        // Each pair should have (name, pubkey)
        for (name, pubkey) in &pubkey_pairs {
            assert!(wallet_names.contains(name));
            assert_eq!(pubkey.to_bytes().len(), 32);
        }
    }

    #[test]
    fn test_get_available_wallets() {
        let mut manager = WalletManager::new();

        // Generate some wallets
        manager.batch_generate_auto_named(5).unwrap();

        // All should be available initially
        let available = manager.get_available_wallets(None);
        assert_eq!(available.len(), 5);

        // Test with limit
        let limited = manager.get_available_wallets(Some(3));
        assert_eq!(limited.len(), 3);
    }

    #[test]
    fn test_assign_and_release_wallets() {
        let mut manager = WalletManager::new();

        let wallet_names = manager.batch_generate("worker", 3).unwrap();

        // Assign to task
        let assign_result = manager.assign_to_task(&wallet_names, "mining_task");
        assert!(assign_result.is_ok());

        // Should have fewer available wallets
        let available = manager.get_available_wallets(None);
        assert_eq!(available.len(), 0); // All assigned

        // Release from task
        let release_result = manager.release_from_task("mining_task");
        assert!(release_result.is_ok());

        let released_wallets = release_result.unwrap();
        assert_eq!(released_wallets.len(), 3);

        // Should be available again
        let available_after = manager.get_available_wallets(None);
        assert_eq!(available_after.len(), 3);
    }

    #[test]
    fn test_assign_nonexistent_wallets_fails() {
        let mut manager = WalletManager::new();

        let fake_wallets = vec!["nonexistent1".to_string(), "nonexistent2".to_string()];
        let result = manager.assign_to_task(&fake_wallets, "some_task");

        assert!(result.is_err());
    }

    #[test]
    fn test_release_nonexistent_task_fails() {
        let mut manager = WalletManager::new();

        let result = manager.release_from_task("nonexistent_task");
        assert!(result.is_err());
    }
}