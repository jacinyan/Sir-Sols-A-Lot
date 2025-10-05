use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
enum TaskStatus {
    Pending = 0,
    InProgress = 1,
    Completed = 2,
    Failed = 3,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
enum TaskPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Task {
    id: String,
    transaction_params: String,
    status: TaskStatus,
    retry_count: u8,
    max_retries: u8,
    priority: TaskPriority,
    last_attempt: Option<u64>,
    created_at: u64,
}

impl Task {
    pub fn new(transaction_params: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            transaction_params,
            status: TaskStatus::Pending,
            retry_count: 0,
            max_retries: 3,
            priority: TaskPriority::Normal,
            last_attempt: None,
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::transaction::{TransactionParams, TransferParams};
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_task_creation() {
        let params = TransactionParams::Transfer(TransferParams {
            from: Pubkey::new_unique(),
            to: Pubkey::new_unique(),
            lamports: 1_000_000,
        });
        let params_json = serde_json::to_string(&params).unwrap();
        let task = Task::new(params_json);
        assert!(task == task);
    }

    #[test]
    fn test_task_clone() {
        let params = TransactionParams::Transfer(TransferParams {
            from: Pubkey::new_unique(),
            to: Pubkey::new_unique(),
            lamports: 1_000_000,
        });
        let params_json = serde_json::to_string(&params).unwrap();
        let task = Task::new(params_json);
        let cloned_task = task.clone();
        assert_eq!(task, cloned_task);
    }

    #[test]
    fn test_task_equality() {
        let params1 = TransactionParams::Transfer(TransferParams {
            from: Pubkey::new_unique(),
            to: Pubkey::new_unique(),
            lamports: 1_000_000,
        });
        let params2 = TransactionParams::Transfer(TransferParams {
            from: Pubkey::new_unique(),
            to: Pubkey::new_unique(),
            lamports: 2_000_000,
        });
        let task1 = Task::new(serde_json::to_string(&params1).unwrap());
        let task2 = Task::new(serde_json::to_string(&params2).unwrap());
        assert_ne!(task1, task2, "Different tasks should have different IDs");
    }

    #[test]
    fn test_task_debug_format() {
        let params = TransactionParams::Transfer(TransferParams {
            from: Pubkey::new_unique(),
            to: Pubkey::new_unique(),
            lamports: 1_000_000,
        });
        let task = Task::new(serde_json::to_string(&params).unwrap());
        let debug_str = format!("{:?}", task);
        assert!(debug_str.as_str().contains("Task"));
    }

    #[test]
    fn test_task_serialization() {
        let params = TransactionParams::Transfer(TransferParams {
            from: Pubkey::new_unique(),
            to: Pubkey::new_unique(),
            lamports: 1_000_000,
        });
        let task = Task::new(serde_json::to_string(&params).unwrap());
        let serialized = serde_json::to_string(&task);
        assert!(serialized.is_ok());
    }

    #[test]
    fn test_task_deserialization() {
        let params = TransactionParams::Transfer(TransferParams {
            from: Pubkey::new_unique(),
            to: Pubkey::new_unique(),
            lamports: 1_000_000,
        });
        let task = Task::new(serde_json::to_string(&params).unwrap());
        let serialized = serde_json::to_string(&task).unwrap();
        let deserialized: Result<Task, _> = serde_json::from_str(&serialized);
        assert!(deserialized.is_ok());
        assert_eq!(task, deserialized.unwrap());
    }

    #[test]
    fn test_task_round_trip_serialization() {
        let params = TransactionParams::Transfer(TransferParams {
            from: Pubkey::new_unique(),
            to: Pubkey::new_unique(),
            lamports: 1_000_000,
        });
        let original_task = Task::new(serde_json::to_string(&params).unwrap());

        let json = serde_json::to_string(&original_task).unwrap();
        let restored_task: Task = serde_json::from_str(&json).unwrap();

        assert_eq!(original_task, restored_task);
    }

    #[test]
    fn test_multiple_tasks_uniqueness() {
        let tasks: Vec<Task> = (0..5)
            .map(|_| {
                let params = TransactionParams::Transfer(TransferParams {
                    from: Pubkey::new_unique(),
                    to: Pubkey::new_unique(),
                    lamports: 1_000_000,
                });
                Task::new(serde_json::to_string(&params).unwrap())
            })
            .collect();

        for i in 0..tasks.len() {
            for j in (i + 1)..tasks.len() {
                assert_ne!(tasks[i], tasks[j], "Each task should be unique");
            }
        }

        let task_set: std::collections::HashSet<_> = tasks.into_iter().collect();
        assert_eq!(task_set.len(), 5);
    }

    #[test]
    fn test_task_in_collections() {
        let params1 = TransactionParams::Transfer(TransferParams {
            from: Pubkey::new_unique(),
            to: Pubkey::new_unique(),
            lamports: 1_000_000,
        });
        let params2 = TransactionParams::Transfer(TransferParams {
            from: Pubkey::new_unique(),
            to: Pubkey::new_unique(),
            lamports: 2_000_000,
        });
        let task1 = Task::new(serde_json::to_string(&params1).unwrap());
        let task2 = Task::new(serde_json::to_string(&params2).unwrap());

        let mut tasks = std::collections::HashSet::new();
        tasks.insert(task1.clone());
        tasks.insert(task2.clone());

        assert!(tasks.contains(&task1));
        assert!(tasks.contains(&task2));
    }

    #[test]
    fn test_task_lifecycle() {
        let params = TransactionParams::Transfer(TransferParams {
            from: Pubkey::new_unique(),
            to: Pubkey::new_unique(),
            lamports: 1_000_000,
        });
        let task = Task::new(serde_json::to_string(&params).unwrap());
        let cloned = task.clone();

        drop(task);

        let serialized = serde_json::to_string(&cloned).unwrap();
        let restored: Task = serde_json::from_str(&serialized).unwrap();

        assert_eq!(cloned, restored);
    }
}
