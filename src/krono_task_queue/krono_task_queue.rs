use std::collections::VecDeque;
use crate::krono_task_queue::KronoTaskQueueTraits;
use crate::threads::GreenThread;

pub struct KronoTaskQueue {
    queue: VecDeque<GreenThread>
}


impl KronoTaskQueueTraits for KronoTaskQueue {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, thread: GreenThread) {
        self.queue.push_back(thread);
    }

    fn pop(&mut self) -> Option<GreenThread> {
        self.queue.pop_front()
    }

    fn len(&self) -> usize {
        self.queue.len()
    }

    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
