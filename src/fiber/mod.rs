pub mod context;
pub mod stack;
use crate::scheduler;
use crate::util::FnBox;
use std::alloc;
use std::mem;
use std::ptr;

const DEFAULT_STACK_PAGES: usize = 64;

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
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn mark_entry(&mut self) -> FiberStateGuard {
        *self = FiberState::Entry;
        FiberStateGuard { state: self }
    }

    #[inline]
    pub fn mark_blocking(&mut self) -> FiberStateGuard {
        *self = FiberState::Blocking;
        FiberStateGuard { state: self }
    }

    #[inline]
    fn reset(&mut self) {
        *self = FiberState::Init;
    }
}

pub struct FiberStateGuard<'s> {
    state: &'s mut FiberState,
}

impl<'s> Drop for FiberStateGuard<'s> {
    #[inline]
    fn drop(&mut self) {
        self.state.reset();
    }
}

pub struct Fiber<'c> {
    ctx: context::Context,
    stack: stack::Stack,
    f: Option<Box<FnBox + 'c>>, // FIXME waiting for Box<FnOnce>
    state: FiberState,
    // TODO &mut scheduler
}

impl<'c> Fiber<'c> {
    extern "C" fn entrance(arg: *mut u8) -> ! {
        let mut fiber = unsafe { Box::from_raw(arg as *mut Fiber) };

        let _guard = fiber.state.mark_entry();
        if let Some(f) = fiber.f.take() {
            f.call_box();
        }

        // TODO task steal from other fibers

        // TODO fiber is dying, move self to dying list

        // TODO switch to idel fiber

        unreachable!();
    }

    #[inline]
    pub fn new<F: FnOnce() + 'c>(pages: usize, f: F) -> Option<Box<Fiber<'c>>> {
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
                fiber.f = Some(Box::new(f));
                fiber.state = FiberState::new();
                Some(unsafe { Box::from_raw(this) })
            })
        })
    }

    #[inline]
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
    //#[test]
    //#[should_panic]
    //fn test_fiber_switch() {
    //    let mut idle = Fiber::new(0, || {}).unwrap();
    //    let mut test = Fiber::new(64, move || {
    //        panic!();
    //    }).unwrap();

    //    idle.switch(&mut test);
    //}
}
