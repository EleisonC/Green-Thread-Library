use crate::threads::{GreenThread, ThreadId};
use anyhow::{Error, Result};

pub trait FluxSchedulerTrait {

    // create a new scheduler
    fn new(max_threads: usize) -> Self;

    fn add_thread(&mut self, thread: GreenThread) -> Result<ThreadId, Error>;

    fn run(&mut self);

    fn wait(&mut self, thread: GreenThread);

    fn wake(&mut self, thread_io: ThreadId);

    fn yield_current_thread(&mut self);
}