use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

/// Represents different types of transaction parameters that can be serialized
/// and stored in tasks for delayed transaction building
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum TransactionParams {
    Transfer(TransferParams),
    CreateAccount(CreateAccountParams),
    BatchTransfer(BatchTransferParams),
}

/// Parameters for a simple SOL transfer transaction
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TransferParams {
    pub from: Pubkey,
    pub to: Pubkey,
    pub lamports: u64,
}

/// Parameters for creating a new account on Solana
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CreateAccountParams {
    pub payer: Pubkey,
    pub new_account: Pubkey,
    pub owner: Pubkey,
    pub lamports: u64,
    pub space: u64,
}

/// Parameters for batch transfer operations (multiple transfers in one transaction)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct BatchTransferParams {
    pub from: Pubkey,
    pub transfers: Vec<(Pubkey, u64)>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_transfer_params_serialization() {
        let params = TransactionParams::Transfer(TransferParams {
            from: Pubkey::new_unique(),
            to: Pubkey::new_unique(),
            lamports: 1_000_000,
        });

        let json = serde_json::to_string(&params).unwrap();
        let deserialized: TransactionParams = serde_json::from_str(&json).unwrap();

        assert_eq!(params, deserialized);
    }

    #[test]
    fn test_create_account_params_serialization() {
        let params = TransactionParams::CreateAccount(CreateAccountParams {
            payer: Pubkey::new_unique(),
            new_account: Pubkey::new_unique(),
            owner: Pubkey::new_unique(),
            lamports: 1_000_000,
            space: 1024,
        });

        let json = serde_json::to_string(&params).unwrap();
        let deserialized: TransactionParams = serde_json::from_str(&json).unwrap();

        assert_eq!(params, deserialized);
    }

    #[test]
    fn test_batch_transfer_params_serialization() {
        let params = TransactionParams::BatchTransfer(BatchTransferParams {
            from: Pubkey::new_unique(),
            transfers: vec![
                (Pubkey::new_unique(), 100_000),
                (Pubkey::new_unique(), 200_000),
            ],
        });

        let json = serde_json::to_string(&params).unwrap();
        let deserialized: TransactionParams = serde_json::from_str(&json).unwrap();

        assert_eq!(params, deserialized);
    }

    #[test]
    fn test_tagged_enum_format() {
        let params = TransactionParams::Transfer(TransferParams {
            from: Pubkey::new_unique(),
            to: Pubkey::new_unique(),
            lamports: 500_000,
        });

        let json = serde_json::to_string(&params).unwrap();
        // Should contain "type": "Transfer" due to #[serde(tag = "type")]
        assert!(json.contains(r#""type":"Transfer"#));
    }
}
