use std::fmt;
use std::mem;

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

pub type EntranceFn = extern "C" fn(p: *mut u8) -> !;

extern "C" {
    #[inline(never)]
    fn context_init(ctx: *mut Context, entrance: EntranceFn, arg: *mut u8, stack_base: *mut u8);

    #[inline(never)]
    fn context_switch(to: *mut Context, from: *mut Context);
}

impl Context {
    #[inline(always)]
    pub unsafe fn new(entrance: EntranceFn, arg: *mut u8, stack_base: *mut u8) -> Context {
        let mut ctx = mem::uninitialized();
        context_init(&mut ctx as *mut _, entrance, arg, stack_base);
        ctx
    }

    #[inline(always)]
    pub unsafe fn switch(&mut self, to: &mut Self) {
        context_switch(to as *mut _, self as *mut _);
    }
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(
                f,
                "rdi {:x}\nrsi {:x}\nrdx {:x}\nrcx {:x}\nr8 {:x}\nr9 {:x}\n\
                 rax {:x}\n rbx {:x}\nr10 {:x}\nrsp {:x}\nrbp {:x}\nr11 {:x}\
                 \nr12 {:x}\nr13 {:x}\nr14 {:x}\nr15 {:x}\nrip {:x}",
                self.rdi,
                self.rsi,
                self.rdx,
                self.rcx,
                self.r8,
                self.r9,
                self.rax,
                self.rbx,
                self.r10,
                self.rsp,
                self.rbp,
                self.r11,
                self.r12,
                self.r13,
                self.r14,
                self.r15,
                self.rip
            )
        }
    }
}
