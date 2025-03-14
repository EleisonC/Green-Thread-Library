use anyhow::{Context, Result};
use crate::flux_memory::{FluxMemoryManager, FluxMemoryTrait};
use crate::flux_scheduler::{FluxScheduler, FluxSchedulerTrait};
use crate::runtime::RuntimeTraits;
use crate::threads::{GreenThread, StaticThreadFn, ThreadId};

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
    scheduler:  FluxScheduler,
    memory_manager: FluxMemoryManager,
    io_uring: bool
}


impl RuntimeCore {
    pub fn new() -> Result<Self> {
        Self::with_config(RuntimeConfig::default()).with_context(|| "failed to create runtime core")
    }

    pub fn with_config(config: RuntimeConfig) -> Result<Self> {
        let memory_manager = FluxMemoryManager::new(config.stack_size);
        let scheduler = FluxScheduler::new(config.max_threads);

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
    unsafe fn spawn(&mut self, f: StaticThreadFn) -> Result<ThreadId> {
        // Implementation will create a new green thread, allocate stack,
        // and schedule it to be run
        let alloc_stack = self.memory_manager.allocate_stack();
        let  new_green_thread = GreenThread::new(
            alloc_stack,
            self.config.stack_size,
            f
        )?;

        let th_id = new_green_thread.thread_id;
        self.scheduler.add_thread(new_green_thread)?;
        Ok(th_id)
    }
    fn run(&mut self) -> Result<()> {
        // Start the scheduler and run until all threads are done
        self.scheduler.run();
        Ok(())
    }
    fn yield_now(&mut self) {
        // perform a context switch back to the scheduler
        self.scheduler.yield_current_thread();
    }
}


