use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
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

    #[test]
    fn test_task_creation() {
        let task = Task::new("test_params".to_string());
        assert!(task == task);
    }

    #[test]
    fn test_task_clone() {
        let task = Task::new("test_params".to_string());
        let cloned_task = task.clone();
        assert_eq!(task, cloned_task);
    }

    #[test]
    fn test_task_equality() {
        let task1 = Task::new("test_params".to_string());
        let task2 = Task::new("test_params".to_string());
        assert_ne!(task1, task2, "Different tasks should have different IDs");
    }

    #[test]
    fn test_task_debug_format() {
        let task = Task::new("test_params".to_string());
        let debug_str = format!("{:?}", task);
        assert!(debug_str.as_str().contains("Task"));
    }

    #[test]
    fn test_task_serialization() {
        let task = Task::new("test_params".to_string());
        let serialized = to_string(&task);
        assert!(serialized.is_ok());
    }

    #[test]
    fn test_task_deserialization() {
        let task = Task::new("test_params".to_string());
        let serialized = to_string(&task).unwrap();
        let deserialized: Result<Task, _> = from_str(&serialized);
        assert!(deserialized.is_ok());
        assert_eq!(task, deserialized.unwrap());
    }

    #[test]
    fn test_task_round_trip_serialization() {
        let original_task = Task::new("test_params".to_string());

        let json = to_string(&original_task).unwrap();
        let restored_task: Task = from_str(&json).unwrap();

        assert_eq!(original_task, restored_task);
    }

    #[test]
    fn test_multiple_tasks_uniqueness() {
        let tasks: Vec<Task> = (0..5)
            .map(|_| Task::new("test_params".to_string()))
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
        let task1 = Task::new("test_params".to_string());
        let task2 = Task::new("test_params".to_string());

        let mut tasks = std::collections::HashSet::new();
        tasks.insert(task1.clone());
        tasks.insert(task2.clone());

        assert!(tasks.contains(&task1));
        assert!(tasks.contains(&task2));
    }

    #[test]
    fn test_task_lifecycle() {
        let task = Task::new("test_params".to_string());
        let cloned = task.clone();

        drop(task);

        let serialized = to_string(&cloned).unwrap();
        let restored: Task = from_str(&serialized).unwrap();

        assert_eq!(cloned, restored);
    }
}
