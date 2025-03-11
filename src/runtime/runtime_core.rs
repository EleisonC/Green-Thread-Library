use anyhow::{Context, Result};
use crate::runtime::RuntimeTraits;
use crate::threads::{StaticThreadFn, ThreadId};

pub struct RuntimeConfig {
    // max number of green threads
    pub max_threads: usize,
    // stack size allocated to each green thread
    pub stack_size: usize,
    // this will be false at the start
    pub use_io_uring: bool
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            max_threads: 1000,
            stack_size: 64 * 1024, // 64 KB
            use_io_uring: false
        }
    }
}

pub struct RuntimeCore {
    config: RuntimeConfig,
    scheduler:  Scheduler,
    memory_manager: MemoryManager,
    io_uring: bool
}


impl RuntimeCore {
    pub fn new() -> Result<Self> {
        Self::with_config(RuntimeConfig::default()).with_context(|| "failed to create runtime core")
    }

    pub fn with_config(config: RuntimeConfig) -> Result<Self> {
        let memory_manager = MemoryManager::new(config.stack_size);
        let scheduler = Scheduler::new(config.max_threads);

        let runtime = Self {
            config,
            scheduler,
            memory_manager,
            io_uring: false
        };
        Ok(runtime)
    }
}

impl RuntimeTraits for RuntimeCore {
    fn spawn(&mut self, f: StaticThreadFn) -> Result<ThreadId> {
        // Implementation will create a new green thread, allocate stack,
        // and schedule it to be run
        todo!()
    }
    fn run(&mut self) -> Result<()> {
        // Start the scheduler and run until all threads are done
        todo!()
    }
    fn yield_now(&self) {
        // perform a context switch back to the scheduler
        todo!()
    }
}


