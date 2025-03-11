use std::collections::HashMap;
use anyhow::{bail, Error};
use crate::flux_scheduler::FluxSchedulerTrait;
use crate::krono_context::KronoContext;
use crate::krono_task_queue::{KronoTaskQueue, KronoTaskQueueTraits};
use crate::threads::{GreenThread, ThreadId, ThreadState};

pub struct FluxScheduler {
    ready_queue: KronoTaskQueue,
    waiting_threads: HashMap<ThreadId, GreenThread>,
    current_thread: Option<ThreadId>,
    scheduler_context: KronoContext,
    running: bool,
    // TT number of active threads (ready, waiting, running)
    thread_count: usize,
    // max num of threads allowed
    max_threads: usize,
}

impl FluxSchedulerTrait for FluxScheduler {
    fn new(max_threads: usize) -> Self {
        #[cfg(target_arch = "aarch64")]
        let mut zero_context = KronoContext {
            x19: 0,
            x20: 0,
            x21: 0,
            x22: 0,
            x23: 0,
            x24: 0,
            x25: 0,
            x26: 0,
            x27: 0,
            x28: 0,
            x29: 0,
            sp: 0,
            lr: 0,
        };
        Self {
            ready_queue: KronoTaskQueue::new(),
            waiting_threads: HashMap::new(),
            current_thread: None,
            scheduler_context: zero_context,
            running: false,
            thread_count: 0,
            max_threads,
        }
    }

    fn add_thread(&mut self, thread: GreenThread) -> Result<ThreadId, Error> {
        if self.thread_count >= self.max_threads {
            bail!("Max threads reached");
        }

        let thread_id = thread.thread_id;
        self.ready_queue.push(thread);
        self.thread_count += 1;
        Ok(thread_id)
    }

    fn run (&mut self) {
        if self.running {
            return;
        }

        self.running = true;
        // loging point

        while !self.ready_queue.is_empty() {
            let mut thread = self.ready_queue.pop().unwrap();
            let thread_id = thread.thread_id;

            // update current thread val
            self.current_thread = Some(thread_id);

            // switch to the thread
            thread.switch_to(&mut self.scheduler_context);

            // reset current thread
            self.current_thread = None;

            // manage thread states
            match thread.state {
                ThreadState::Ready => {
                    self.ready_queue.push(thread);
                },
                ThreadState::Waiting => {
                    self.waiting_threads.insert(thread.thread_id, thread);
                },
                ThreadState::Completed => {
                    self.thread_count -= 1;
                },
                ThreadState::Running => {
                    // Send it back to the ready queue
                    thread.state = ThreadState::Ready;
                    self.ready_queue.push(thread);
                }
            }


        }
        self.running = false;
    }

    /// Mark the current thread as waiting and yield back to the scheduler
    fn wait(&mut self) {
        if let Some(thread_id) = self.current_thread {
            let mut thread = self.waiting_threads.remove(&thread_id).unwrap();
            thread.state = ThreadState::Waiting;
            // switch to the thread
            thread.switch_to(&mut self.scheduler_context);
        }
    }

    /// Wake up a waiting thread and move it to the ready queue
    fn wake(&mut self, thread: ThreadId) {
        if let Some(mut thread) = self.waiting_threads.remove(&thread) {
            thread.state = ThreadState::Ready;
            self.ready_queue.push(thread);
        }
    }

    /// Yield the current thread back to the scheduler
    fn yield_current_thread(&mut self) {
        if let Some(thread_id) = self.current_thread {
            // Mark current thread, as ready.
            // switch to the scheduler context
            let mut thread = self.waiting_threads.remove(&thread_id).unwrap();
            thread.state = ThreadState::Ready;
            // switch to the thread
            thread.switch_to(&mut self.scheduler_context);
        }
    }

}
