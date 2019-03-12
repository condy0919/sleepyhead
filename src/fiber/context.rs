use std::os::raw::c_void;

#[repr(C)]
#[repr(packed)]
pub struct Context {
    rdi: u64,
    rsi: u64,
    rdx: u64,
    rcx: u64,
    r8: u64,
    r9: u64,
    rax: u64,
    rbx: u64,
    r10: u64,
    rsp: u64,
    rbp: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    rip: u64,
}

// TODO void* -> Fiber&
pub type EntranceFn = extern "C" fn(p: *mut c_void) -> !;

extern "C" {
    #[inline(never)]
    fn context_init(ctx: *mut Context, entrance: EntranceFn, arg: *mut c_void, stack_base: *mut c_void);

    #[inline(never)]
    fn context_switch(to: *mut Context, from: *mut Context);
}
