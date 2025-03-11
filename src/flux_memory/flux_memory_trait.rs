pub trait  FluxMemoryTrait {

    /// create a new memory manager with stack_size
    fn new(stack_size: usize) -> Self;

    /// allocate new stack memory or re-use stack available memory
    fn allocate_stack(&mut self) -> *mut u8;

    /// Return a stack to the pool for reuse
    fn deallocate_stack(&mut self, ptr: *mut u8);
}