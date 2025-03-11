mod context_traits;
mod krono_context;

use std::arch::global_asm;
use std::ffi::c_void;
pub use context_traits::KronoContextOps;
pub use krono_context::KronoContext;


#[cfg(target_arch = "aarch64")]
global_asm!(include_str!("./asm/aarch64.s"));

extern "C" {
    fn context_switch(from: *mut KronoContext, to: *const KronoContext);
}

#[cfg(target_arch = "aarch64")]
impl KronoContextOps for KronoContext {
    unsafe fn new(
        stack: *mut u8,
        stack_size: usize,
        entry: extern "C" fn(*mut c_void),
        arg: *mut c_void,
    ) -> Self {
        // Initialize registers to zero.
        let mut ctx = Self {
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

        // calc to get to top of the stack
        let stack_top = (stack as usize) + stack_size;

        // align the stack 16 bytes for ABI (--Flatten building top)
        // solve for equal 16 byte chunks from the start to top

        let stack_aligned = (stack_top - 8) & !15;
        let stack_ptr = stack_aligned as *mut usize;

        // prepare spots for entry and args
        *stack_ptr.offset(-1) = entry as usize;
        *stack_ptr.offset(-2) = arg as usize;
        // Set the stack pointer.
        ctx.sp = stack_ptr.offset(-2) as usize;

        ctx
    }
    unsafe fn swap(&mut self, other: &mut Self) {
        context_switch(self as *mut _, other as *const _);
    }
}

#[cfg(target_arch = "x86_64")]
impl KronoContextOps for KronoContext {
    unsafe fn new(
        stack: *mut u8,
        stack_size: usize,
        entry: extern "C" fn() -> *mut c_void,
        arg: *mut c_void,
    ) -> Self {
        let mut ctx = Self {
            rbp: 0,
            rbx: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rsp: 0
        };

        // calc to get to top of the stack
        let stack_top = (stack as usize) + stack_size;

        // align the stack 16 bytes for ABI (--Flatten building top)
        // solve for equal 16 byte chunks from the start

        let stack_aligned = (stack_top - 8) & !15;
        let stack_ptr = stack_aligned as *mut usize;

        // prepare spots for entry and args
        *stack_ptr.offset(-1) = entry as usize;
        *stack_ptr.offset(-2) = arg as usize;

        ctx.rsp = stack_ptr.offset(-2) as usize;

        ctx
    }
    unsafe fn swap(&mut self, other: &mut Self) {
        context_switch_x86(self as *mut _, other as *const _);
    }
}

#[cfg(target_arch = "x86_64")]
extern "C" {
    fn context_switch(from: *mut Context, to: *const Context);
}



