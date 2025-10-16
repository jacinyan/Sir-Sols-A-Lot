use crate::task::Task;
use crossbeam::queue::SegQueue;

pub struct TaskQueue {
    inner: SegQueue<Task>,
    max_size: Option<usize>,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            inner: SegQueue::new(),
            max_size: None,
        }
    }

    pub fn enqueue(&mut self, task: Task) -> Result<(), &'static str> {
        if let Some(max) = self.max_size {
            if self.len() >= max {
                return Err("Queue is bounded and it's full");
            }
        }
        self.inner.push(task);
        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<Task> {
        self.inner.pop()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}
