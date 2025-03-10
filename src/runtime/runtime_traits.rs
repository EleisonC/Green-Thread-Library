use anyhow::Result;
pub trait RuntimeTraits {
    /// Spawn a new green thread to execute the given function
    fn spawn(&mut self, f: ThreadFunction) -> Result<ThreadId>;
    /// Run the scheduler until all threads complete
    fn run(&mut self) -> Result<()>;
    /// Yield execution from the current green thread back to the scheduler
    fn yield_now(&self);
}