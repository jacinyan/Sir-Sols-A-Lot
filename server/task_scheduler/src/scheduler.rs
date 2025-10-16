use crate::{queue::TaskQueue, task::Task};

pub struct TaskScheduler {
    queue: TaskQueue,
}

impl TaskScheduler {
    pub fn new() -> Self {
        Self {
            queue: TaskQueue::new(),
        }
    }

    pub fn schedule_task(&mut self, task: Task) -> Result<(), &'static str> {
        match self.queue.enqueue(task) {
            Err(e) => Err(e),
            Ok(_) => Ok(()),
        }
    }

    pub fn execute_next(&mut self) -> Option<Task> {
        todo!("Execute next task")
    }

    pub fn get_pending_count(&self) -> usize {
        self.queue.len()
    }

    pub fn clear_all(&mut self) {
        todo!("Clear all tasks")
    }

    pub fn is_empty(&self) -> bool {
        self.queue.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::transaction::{TransactionParams, TransferParams};
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_create_scheduler() {
        let scheduler = TaskScheduler::new();
        assert!(scheduler.is_empty());
        assert_eq!(scheduler.get_pending_count(), 0);
    }

    #[test]
    fn test_schedule_single_task() {
        let mut scheduler = TaskScheduler::new();
        let params = TransactionParams::Transfer(TransferParams {
            from: Pubkey::new_unique(),
            to: Pubkey::new_unique(),
            lamports: 1_000_000,
        });
        let task = Task::new(serde_json::to_string(&params).unwrap());

        scheduler.schedule_task(task);

        assert!(!scheduler.is_empty());
        assert_eq!(scheduler.get_pending_count(), 1);
    }

    #[test]
    fn test_schedule_multiple_tasks() {
        let mut scheduler = TaskScheduler::new();

        for i in 0..5 {
            let params = TransactionParams::Transfer(TransferParams {
                from: Pubkey::new_unique(),
                to: Pubkey::new_unique(),
                lamports: (i + 1) * 100_000,
            });
            scheduler.schedule_task(Task::new(serde_json::to_string(&params).unwrap()));
        }

        assert_eq!(scheduler.get_pending_count(), 5);
        assert!(!scheduler.is_empty());
    }

    #[test]
    fn test_execute_single_task() {
        let mut scheduler = TaskScheduler::new();
        let params = TransactionParams::Transfer(TransferParams {
            from: Pubkey::new_unique(),
            to: Pubkey::new_unique(),
            lamports: 1_000_000,
        });
        let task = Task::new(serde_json::to_string(&params).unwrap());

        scheduler.schedule_task(task.clone());
        let executed_task = scheduler.execute_next();

        assert!(executed_task.is_some());
        assert_eq!(executed_task.unwrap(), task);
        assert!(scheduler.is_empty());
        assert_eq!(scheduler.get_pending_count(), 0);
    }

    #[test]
    fn test_execute_multiple_tasks_fifo() {
        let mut scheduler = TaskScheduler::new();

        // Schedule multiple tasks
        for i in 0..3 {
            let params = TransactionParams::Transfer(TransferParams {
                from: Pubkey::new_unique(),
                to: Pubkey::new_unique(),
                lamports: (i + 1) * 100_000,
            });
            let task = Task::new(serde_json::to_string(&params).unwrap());
            scheduler.schedule_task(task);
        }

        // Execute all tasks and verify FIFO order
        let mut executed_count = 0;
        while let Some(_task) = scheduler.execute_next() {
            executed_count += 1;
        }

        assert_eq!(executed_count, 3);
        assert!(scheduler.is_empty());
    }

    #[test]
    fn test_execute_from_empty_scheduler() {
        let mut scheduler = TaskScheduler::new();

        let result = scheduler.execute_next();

        assert!(result.is_none());
        assert!(scheduler.is_empty());
    }

    #[test]
    fn test_clear_all_tasks() {
        let mut scheduler = TaskScheduler::new();

        // Add some tasks
        for i in 0..5 {
            let params = TransactionParams::Transfer(TransferParams {
                from: Pubkey::new_unique(),
                to: Pubkey::new_unique(),
                lamports: (i + 1) * 100_000,
            });
            scheduler.schedule_task(Task::new(serde_json::to_string(&params).unwrap()));
        }

        assert_eq!(scheduler.get_pending_count(), 5);

        scheduler.clear_all();

        assert!(scheduler.is_empty());
        assert_eq!(scheduler.get_pending_count(), 0);
    }

    #[test]
    fn test_scheduler_state_consistency() {
        let mut scheduler = TaskScheduler::new();

        // Test initial state
        assert!(scheduler.is_empty());
        assert_eq!(scheduler.get_pending_count(), 0);

        // Add task
        let params = TransactionParams::Transfer(TransferParams {
            from: Pubkey::new_unique(),
            to: Pubkey::new_unique(),
            lamports: 1_000_000,
        });
        scheduler.schedule_task(Task::new(serde_json::to_string(&params).unwrap()));
        assert!(!scheduler.is_empty());
        assert_eq!(scheduler.get_pending_count(), 1);

        // Execute task
        let _executed = scheduler.execute_next();
        assert!(scheduler.is_empty());
        assert_eq!(scheduler.get_pending_count(), 0);
    }

    #[test]
    fn test_mixed_operations() {
        let mut scheduler = TaskScheduler::new();

        // Schedule some tasks
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
        scheduler.schedule_task(Task::new(serde_json::to_string(&params1).unwrap()));
        scheduler.schedule_task(Task::new(serde_json::to_string(&params2).unwrap()));
        assert_eq!(scheduler.get_pending_count(), 2);

        // Execute one
        let _task = scheduler.execute_next();
        assert_eq!(scheduler.get_pending_count(), 1);

        // Schedule one more
        let params3 = TransactionParams::Transfer(TransferParams {
            from: Pubkey::new_unique(),
            to: Pubkey::new_unique(),
            lamports: 3_000_000,
        });
        scheduler.schedule_task(Task::new(serde_json::to_string(&params3).unwrap()));
        assert_eq!(scheduler.get_pending_count(), 2);

        // Clear all
        scheduler.clear_all();
        assert!(scheduler.is_empty());
        assert_eq!(scheduler.get_pending_count(), 0);
    }
}
