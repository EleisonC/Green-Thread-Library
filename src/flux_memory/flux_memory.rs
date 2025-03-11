use std::collections::VecDeque;
use std::ptr;
use libc::{ self, c_void, size_t };
use crate::flux_memory::flux_memory_trait::FluxMemoryTrait;

pub struct FluxMemoryManager {
    stack_size: usize,
    stack_pool: VecDeque<u8>,
}

impl FluxMemoryTrait for FluxMemoryManager {
    fn new(stack_size: usize) -> Self {
        Self {
            stack_size,
            stack_pool: VecDeque::new(),
        }
    }

    fn allocate_stack(&mut self) -> *mut u8 {
        // attempt to re-use a stack from the pool
        if let Some(stack) = self.stack_pool.pop_front() {
            return stack as *mut u8;
        }

        let stack = unsafe  {
            libc::mmap(
                ptr::null_mut(),
                self.stack_size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
                -1,
                0
            ) as *mut u8
        };

        if stack == libc::MAP_FAILED as *mut u8 {
            panic!("Failed to allocate stack memory");
        }

        stack
    }

    fn deallocate_stack(&mut self, stack: u8) {
        self.stack_pool.push_back(stack);
    }
}