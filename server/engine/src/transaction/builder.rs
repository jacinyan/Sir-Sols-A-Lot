use solana_pubkey::Pubkey;
use solana_sdk::{
    hash::Hash,
    instruction::Instruction,
    message::Message,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction;
use std::sync::Arc;

pub struct TransactionBuilder {
    instructions: Vec<Instruction>,
    recent_blockhash: Option<Hash>,
}

impl TransactionBuilder {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            recent_blockhash: None,
        }
    }

    // Basic transaction
    pub fn add_instruction(&mut self, instruction: Instruction) -> &mut Self {
        self.instructions.push(instruction);

        self
    }

    pub fn set_recent_blockhash(&mut self, blockhash: Hash) -> &mut Self {
        self.recent_blockhash = Some(blockhash);

        self
    }

    pub fn build(&self, signers: &Vec<&Keypair>) -> Result<Transaction, String> {
        // Check external input first
        if signers.is_empty() {
            return Err("No signers provided".to_string());
        }

        // Check internal state
        if self.recent_blockhash.is_none() {
            return Err("Missing recent_blockhash".to_string());
        }
        if self.instructions.is_empty() {
            return Err("No instructions".to_string());
        }

        // Devnet for now, later we can make this configurable
        Ok(Transaction::new(
            signers,
            Message::new(&self.instructions, None),
            self.recent_blockhash.unwrap(),
        ))
    }

    // Common transaction types
    pub fn transfer(
        from: &Keypair,
        to: &Pubkey,
        lamports: u64,
        recent_blockhash: Hash,
    ) -> Result<Transaction, String> {
        let mut builder = TransactionBuilder::new();
        builder
            .add_instruction(instruction::transfer(&from.pubkey(), &to, lamports))
            .set_recent_blockhash(recent_blockhash);

        let signers = vec![from];
        // No error mapping, no custom error types for now
        let result = builder.build(&signers)?;

        Ok(result)
    }

    pub fn create_account(
        payer: &Keypair,
        new_account: &Keypair,
        owner: &Pubkey,
        lamports: u64,
        space: u64,
        recent_blockhash: Hash,
    ) -> Result<Transaction, String> {
        let mut builder = TransactionBuilder::new();
        let instruction = instruction::create_account(
            &payer.pubkey(),
            &new_account.pubkey(),
            lamports,
            space,
            owner,
        );

        builder
            .add_instruction(instruction)
            .set_recent_blockhash(recent_blockhash);

        let signers = vec![payer, new_account];
        let result = builder.build(&signers)?;

        Ok(result)
    }

    // Batch operations
    pub fn batch_transfer(
        from: &Keypair,
        transfers: &[(Pubkey, u64)],
        recent_blockhash: Hash,
    ) -> Result<Transaction, String> {
        todo!("create batch transfer transaction")
    }

    // Transaction validation
    pub fn estimate_fee(&self) -> Result<u64, String> {
        todo!("estimate transaction fee")
    }

    pub fn validate(&self) -> Result<(), String> {
        todo!("validate transaction before sending")
    }

    // Transaction size management
    pub fn instruction_count(&self) -> usize {
        self.instructions.len()
    }

    pub fn estimated_size(&self) -> usize {
        todo!("estimate transaction size in bytes")
    }

    pub fn clear(&mut self) {
        todo!("clear all instructions and signers")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_transaction_builder() {
        let builder = TransactionBuilder::new();
        assert_eq!(builder.instruction_count(), 0);
    }

    #[test]
    fn test_add_instruction() {
        let mut builder = TransactionBuilder::new();
        let from = Keypair::new();
        let to = Keypair::new();

        let instruction = instruction::transfer(&from.pubkey(), &to.pubkey(), 1000000);

        builder.add_instruction(instruction);
        assert_eq!(builder.instruction_count(), 1);
    }

    #[test]
    fn test_add_multiple_instructions() {
        let mut builder = TransactionBuilder::new();
        let from = Keypair::new();
        let to1 = Keypair::new();
        let to2 = Keypair::new();

        let instruction1 = instruction::transfer(&from.pubkey(), &to1.pubkey(), 1000000);
        let instruction2 = instruction::transfer(&from.pubkey(), &to2.pubkey(), 2000000);

        builder
            .add_instruction(instruction1)
            .add_instruction(instruction2);
        assert_eq!(builder.instruction_count(), 2);
    }

    #[test]
    fn test_simple_transfer_transaction() {
        let from = Keypair::new();
        let to = Keypair::new();
        let recent_blockhash = Hash::default();

        let result = TransactionBuilder::transfer(&from, &to.pubkey(), 1000000, recent_blockhash);
        assert!(result.is_ok());

        let transaction = result.unwrap();
        assert_eq!(transaction.message.instructions.len(), 1);
    }

    #[test]
    fn test_transfer_with_zero_lamports_fails() {
        let from = Keypair::new();
        let to = Keypair::new();
        let recent_blockhash = Hash::default();

        let result = TransactionBuilder::transfer(&from, &to.pubkey(), 0, recent_blockhash);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_account_transaction() {
        let payer = Keypair::new();
        let new_account = Keypair::new();
        let owner = Pubkey::new_unique();
        let recent_blockhash = Hash::default();

        let result = TransactionBuilder::create_account(
            &payer,
            &new_account,
            &owner,
            1000000,
            1024,
            recent_blockhash,
        );
        assert!(result.is_ok());

        let transaction = result.unwrap();
        assert!(transaction.message.instructions.len() > 0);
    }

    #[test]
    #[ignore]
    fn test_batch_transfer() {
        let from = Keypair::new();
        let to1 = Pubkey::new_unique();
        let to2 = Pubkey::new_unique();
        let to3 = Pubkey::new_unique();
        let recent_blockhash = Hash::default();

        let transfers = vec![(to1, 1000000), (to2, 2000000), (to3, 3000000)];

        let result = TransactionBuilder::batch_transfer(&from, &transfers, recent_blockhash);
        assert!(result.is_ok());

        let transaction = result.unwrap();
        assert_eq!(transaction.message.instructions.len(), 3);
    }

    #[test]
    #[ignore]
    fn test_batch_transfer_empty_list_fails() {
        let from = Keypair::new();
        let recent_blockhash = Hash::default();
        let transfers = vec![];

        let result = TransactionBuilder::batch_transfer(&from, &transfers, recent_blockhash);
        assert!(result.is_err());
    }

    #[test]
    fn test_build_without_recent_blockhash_fails() {
        let mut builder = TransactionBuilder::new();
        let instruction =
            instruction::transfer(&Keypair::new().pubkey(), &Keypair::new().pubkey(), 1000000);

        builder.add_instruction(instruction);


        let from = Keypair::new();
        let signers = vec![&from];
        let result = builder.build(&signers);
        assert!(result.is_err());
    }

    #[test]
    #[ignore]
    fn test_build_without_instructions_fails() {
        let mut builder = TransactionBuilder::new();
        builder.set_recent_blockhash(Hash::default());
        
        
        let from = Keypair::new();
        let signers = vec![&from];
        let result = builder.build(&signers);
        assert!(result.is_err());
    }

    #[test]
    #[ignore]
    fn test_validate_transaction() {
        let mut builder = TransactionBuilder::new();
        let from = Keypair::new();
        let to = Keypair::new();

        let instruction = instruction::transfer(&from.pubkey(), &to.pubkey(), 1000000);
        builder
            .add_instruction(instruction)
            .set_recent_blockhash(Hash::default());

        let result = builder.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_estimate_transaction_fee() {
        let mut builder = TransactionBuilder::new();
        let instruction =
            instruction::transfer(&Keypair::new().pubkey(), &Keypair::new().pubkey(), 1000000);

        builder.add_instruction(instruction);
        let result = builder.estimate_fee();
        assert!(result.is_ok());

        let fee = result.unwrap();
        assert!(fee > 0);
    }

    #[test]
    fn test_estimated_size() {
        let mut builder = TransactionBuilder::new();
        let initial_size = builder.estimated_size();

        let instruction =
            instruction::transfer(&Keypair::new().pubkey(), &Keypair::new().pubkey(), 1000000);
        builder.add_instruction(instruction);

        let new_size = builder.estimated_size();
        assert!(new_size > initial_size);
    }

    #[test]
    fn test_clear_builder() {
        let mut builder = TransactionBuilder::new();
        let instruction =
            instruction::transfer(&Keypair::new().pubkey(), &Keypair::new().pubkey(), 1000000);

        builder.add_instruction(instruction);
        assert_eq!(builder.instruction_count(), 1);

        builder.clear();
        assert_eq!(builder.instruction_count(), 0);
    }
}
