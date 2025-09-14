use std::collections::HashMap;
use solana_sdk::signature::{Keypair, Signer};
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

    pub fn generate_wallet(&mut self, name: String) -> Result<Pubkey, String> {
        if self.wallets.contains_key(&name) {
            return Err(format!("Wallet '{}' already exists", name));
        }

        let keypair = Keypair::new();
        let pubkey = keypair.pubkey();
        self.wallets.insert(name, keypair);

        Ok(pubkey)
    }

    pub fn import_wallet(&mut self, _name: String, _private_key_bytes: &[u8]) -> Result<Pubkey, String> {
        unimplemented!("wallet import from private key")
    }

    pub fn get_wallet(&self, name: &str) -> Result<&Keypair, String> {
        match self.wallets.get(name) {
            Some(keypair) => Ok(keypair),
            None => Err(format!("Wallet '{}' not found", name))
        }
    }

    pub fn get_pubkey(&self, name: &str) -> Result<Pubkey, String> {
        match self.wallets.get(name) {
            Some(keypair) => Ok(keypair.pubkey()),
            None => Err(format!("Wallet '{}' not found", name))
        }
    }

    pub fn list_wallets(&self) -> Vec<(String, Pubkey)> {
        self.wallets.iter()
            .map(|(name, keypair)| (name.clone(), keypair.pubkey()))
            .collect()
    }

    pub fn remove_wallet(&mut self, name: &str) -> Result<(), String> {
        match self.wallets.remove(name) {
            Some(_) => Ok(()),
            None => Err(format!("Wallet '{}' not found", name))
        }
    }

    pub fn wallet_count(&self) -> usize {
        self.wallets.len()
    }

    // Batch operations for wallet factory
    pub fn batch_generate(&mut self, prefix: &str, count: usize) -> Result<Vec<String>, String> {
        let mut generated_names = Vec::new();

        for i in 0..count {
            let name = format!("{}_{}", prefix, i);
            match self.generate_wallet(name.clone()) {
                Ok(_) => generated_names.push(name),
                Err(e) => return Err(e)
            }
        }

        Ok(generated_names)
    }

    pub fn batch_generate_auto_named(&mut self, count: usize) -> Result<Vec<String>, String> {
        let mut generated_names = Vec::new();

        for i in 0..count {
            let name = format!("wallet_{}", self.counter + i);
            match self.generate_wallet(name.clone()) {
                Ok(_) => generated_names.push(name),
                Err(e) => return Err(e)
            }
        }

        self.counter += count;
        Ok(generated_names)
    }

    pub fn batch_get_pubkeys(&self, wallet_names: &[String]) -> Result<Vec<(String, Pubkey)>, String> {
        let mut result = Vec::new();

        for name in wallet_names {
            match self.get_pubkey(name) {
                Ok(pubkey) => result.push((name.clone(), pubkey)),
                Err(e) => return Err(e)
            }
        }

        Ok(result)
    }
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

}