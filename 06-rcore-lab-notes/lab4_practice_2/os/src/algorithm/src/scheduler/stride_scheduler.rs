//! Stride Scheduling 调度算法

use super::Scheduler;
use super::PriorityQueue;

pub struct StrideScheduler<ThreadType: Clone + Eq> {
    pool: PriorityQueue<ThreadType, usize>,
}

impl<ThreadType: Clone + Eq> Default for StrideScheduler<ThreadType> {
    fn default() -> Self {
        Self {
            pool: PriorityQueue::new(),
        }
    }
}

impl<ThreadType: Clone + Eq> Scheduler<ThreadType> for StrideScheduler<ThreadType> {
    fn add_thread(&mut self, thread: ThreadType, priority: usize) {
            self.pool.push(thread, priority);
    }

    fn get_next(&mut self) -> Option<ThreadType> {
        if let Some(obj) = self.pool.pop() {
            self.pool.push(obj.object.clone(), obj.priority);
            Some(obj.object)
        } else {
            None
        }
    }

    fn remove_thread(&mut self, thread: &ThreadType) {
        self.pool.remove(&thread)
    }

    fn set_priority(&mut self, thread: ThreadType, priority: usize) {
            self.pool.set_priority(thread, priority);
    }
}
