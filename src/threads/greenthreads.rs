use crate::context::Context;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;
use std::task::Context;

pub struct ThreadId(usize);

impl ThreadId {
    pub fn new() -> Self {
        static ID_COUNTER: OnceLock<AtomicUsize> = OnceLock::new();

        let counter = ID_COUNTER.get_or_init(|| AtomicUsize::new(1));

        Self(counter.fetch_add(1, Ordering::SeqCst))
    }

    pub fn th_id(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Ready,
    Running,
    Waiting,
    Completed,
}

pub struct GreenThread {
    pub context: Context,
    pub stack: *mut u8, // memory area used as the thread's stack
    pub stack_size: usize, // size of allocated stack
    pub state: ThreadState,
    pub thread_id: ThreadId,
}


