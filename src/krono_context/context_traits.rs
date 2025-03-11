use std::ffi::c_void;

pub trait KronoContextOps {
    unsafe fn new(  // this unsafe fn because it works with raw pointers & low level stack manipulation
        stack: *mut u8,
        stack_size: usize,
        entry: extern "C" fn() -> *mut c_void,
        arg: *mut c_void
    ) -> Self;

    unsafe fn swap(&mut self, other: &mut Self); // unsafe because it manipulates execution contexts at lo
}