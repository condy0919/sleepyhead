extern crate boxfnonce;

pub mod context;
pub mod stack;
use crate::scheduler;
use boxfnonce::BoxFnOnce;
use context::EntranceFn;
use std::alloc;
use std::mem;
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FiberState {
    Init,
    Entry,
    Blocking,
}

impl Default for FiberState {
    fn default() -> Self {
        FiberState::Init
    }
}

impl FiberState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mark_entry(&mut self) -> FiberStateGuard {
        *self = FiberState::Entry;
        FiberStateGuard { state: self }
    }

    pub fn mark_blocking(&mut self) -> FiberStateGuard {
        *self = FiberState::Blocking;
        FiberStateGuard { state: self }
    }

    fn reset(&mut self) {
        *self = FiberState::Init;
    }
}

pub struct FiberStateGuard<'s> {
    state: &'s mut FiberState,
}

impl<'s> Drop for FiberStateGuard<'s> {
    fn drop(&mut self) {
        self.state.reset();
    }
}

pub struct Fiber<'c> {
    ctx: context::Context,
    stack: stack::Stack,
    f: Option<BoxFnOnce<'c, ()>>, // FIXME waiting for Box<FnOnce>
    state: FiberState,
    // TODO &mut scheduler
}

impl<'a> Fiber<'a> {
    // FIXME How to use EntranceFn instead
    extern "C" fn entrance(arg: *mut u8) -> ! {
        let mut fiber = unsafe { Box::from_raw(arg as *mut Fiber) };

        let _guard = fiber.state.mark_entry();
        if let Some(f) = fiber.f.take() {
            f.call();
        }

        // TODO fiber is dying, move self to dying list

        // TODO switch to idel fiber

        unreachable!();
    }

    pub fn new<'c, F: FnOnce() + 'c>(pages: usize, f: F) -> Option<Box<Fiber<'c>>> {
        stack::Stack::with_pages(pages).and_then(|stk| {
            let layout = alloc::Layout::new::<Fiber>();
            ptr::NonNull::new(unsafe { alloc::alloc(layout) }).and_then(|p| {
                let this = p.as_ptr() as *mut Fiber;
                let fiber: &mut Fiber = unsafe { &mut *this };

                fiber.ctx = unsafe {
                    context::Context::new(
                        Fiber::entrance,
                        this as *mut u8,
                        // 16B alignment
                        stk.base().offset(-2 * (mem::size_of::<*mut u8>() as isize)),
                    )
                };
                fiber.stack = stk;
                fiber.f = Some(BoxFnOnce::from(f));
                fiber.state = FiberState::new();
                Some(unsafe { Box::from_raw(this) })
            })
        })
    }

    pub fn switch(&mut self, to: &mut Self) {
        unsafe { self.ctx.switch(&mut to.ctx) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fiber_state_guard() {
        let mut state = FiberState::new();
        let addr = &mut state as *mut _;

        assert_eq!(state, FiberState::Init);

        // FIXME I know it's ugly, but I can't figure out a safe solution
        unsafe {
            let _guard = state.mark_entry();
            assert_eq!(*addr, FiberState::Entry);
        }
        assert_eq!(state, FiberState::Init);

        unsafe {
            let _guard = state.mark_blocking();
            assert_eq!(*addr, FiberState::Blocking);
        }
        assert_eq!(state, FiberState::Init);
    }

    // FIXME CURRENTLY IT FAILS
    #[test]
    #[should_panic]
    fn test_fiber_switch() {
        let mut idle = Fiber::new(64, || {}).unwrap();
        let mut test = Fiber::new(64, move || {
            panic!();
        }).unwrap();

        idle.switch(&mut test);
    }
}
