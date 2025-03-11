use std::ffi::c_void;

// Define an arch-specific module
#[cfg(target_arch = "x86_64")]
mod arch {
    use std::ffi::c_void;

    #[repr(C)]
    pub struct KronoContext {
        // Callee-saved registers for x86_64
        pub rbp: usize,
        pub rbx: usize,
        pub r12: usize,
        pub r13: usize,
        pub r14: usize,
        pub r15: usize,
        // Stack pointer
        pub rsp: usize,
    }
}

#[cfg(target_arch = "aarch64")]
mod arch {
    pub struct KronoContext {
        // Callee-saved registers for ARM64 (AArch64)
        pub x19: usize,
        pub x20: usize,
        pub x21: usize,
        pub x22: usize,
        pub x23: usize,
        pub x24: usize,
        pub x25: usize,
        pub x26: usize,
        pub x27: usize,
        pub x28: usize,
        pub x29: usize,

        // stack pointer
        pub sp: usize,
        pub lr: usize,
    }
}

pub use arch::KronoContext;
