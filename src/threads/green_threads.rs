use std::sync::atomic::{AtomicUsize, Ordering};
use anyhow::Result;
use std::sync::OnceLock;
use crate::krono_context::{KronoContext, KronoContextOps};

#[derive(Debug, Copy, Eq, PartialEq, Hash)]
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

pub type StaticThreadFn = impl FnOnce() + 'static;
pub type NonStaticThreadFn = impl FnOnce();
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Ready,
    Running,
    Waiting,
    Completed,
}

/// simply an entry point for all threads
extern "C" fn thread_entry(arg: *mut std::ffi::c_void) {
    unsafe {
        let f = Box::from_raw(arg as *mut Box<dyn FnOnce()>);

        // call the function
        f();

        // Mark the thread as completed and Switch back to scheduler
        todo!();

        unreachable!("Thread function returned to entry point");
    }
}

pub struct GreenThread {
    pub context: KronoContext,
    pub stack: *mut u8, // memory area used as the thread's stack
    pub stack_size: usize, // size of allocated stack
    pub state: ThreadState,
    pub thread_id: ThreadId,
}

impl GreenThread {
    pub unsafe fn new(stack: *mut u8,
                      stack_size: usize,
                      f: StaticThreadFn
    ) -> Result<Self> {
        let thread_id = ThreadId::new();

        let context = KronoContext::new(
            stack,
            stack_size,
            thread_entry,
            Box::into_raw(Box::new(f))
              as *mut libc::c_void);

        Ok(Self {
            context,
            stack,
            stack_size,
            state: ThreadState::Ready,
            thread_id
        })
    }

    pub fn switch_to(&mut self, from: &mut KronoContext) {
        self.state = ThreadState::Running;
        unsafe {
            KronoContext::swap(from, &mut self.context);
        }
    }
}

impl Drop for GreenThread {
    fn drop(&mut self) {}
}
