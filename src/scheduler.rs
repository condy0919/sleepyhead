use crate::fiber;
use crate::io::{adaptor, monitor};
use crate::util::spinlock;
use crate::util::FnBox;
use std::mem;
use std::cell::RefCell;
use std::collections::VecDeque;

thread_local! {
    #[allow(non_upper_case_globals)]
    pub static current_limbo: RefCell<fiber::Fiber<'static>> = unsafe { mem::uninitialized() };
}

pub struct Scheduler<'m, 'f> {
    max_n_fiber: usize,
    iomon: &'m mut monitor::Monitor,
    running: VecDeque<Box<fiber::Fiber<'f>>>,
    blocking: VecDeque<Box<fiber::Fiber<'f>>>,
    dying: VecDeque<Box<fiber::Fiber<'f>>>,
    lock: spinlock::SpinLock,
    reqs: Vec<Box<FnBox + 'f>>,
    evch: adaptor::Adaptor,
    // TODO io extension points
}

impl<'m, 'f> Scheduler<'m, 'f> {
    pub fn new(max_n_fiber: usize, iomon: &'m mut monitor::Monitor) -> Self {
        let evch = adaptor::Adaptor::with_nonblock();
        iomon.register_ltrd(evch.read_endpoint() as u64 | monitor::EVENT_CHANNEL_FLAG);

        // TODO io extension points setup

        Scheduler {
            max_n_fiber,
            iomon,
            running: VecDeque::new(),
            blocking: VecDeque::new(),
            dying: VecDeque::new(),
            lock: spinlock::SpinLock::new(),
            reqs: Vec::new(),
            evch,
        }
    }

    pub fn spawn<F: FnOnce() + 'f>(&mut self, f: F) -> bool {
        if self.lock.try_lock() {
            self.reqs.push(Box::new(f));
            self.lock.unlock();
            true
        } else {
            let fptr = Box::into_raw(Box::new(f));
            self.evch.send(fptr as *mut u8 as usize)
                .map(|sz| sz == mem::size_of::<usize>())
                .unwrap_or(false)
        }
    }

    // TODO mainloop
    pub fn mainloop(&mut self) {
        loop {
            // 0. TODO deliver asynck

            // 1. fetch quick reqs
            self.lock.lock();
            while let Some(f) = self.reqs.pop() {
                if let Some(fiber) = fiber::Fiber::from_boxed(fiber::DEFAULT_STACK_PAGES, f) {
                    self.running.push_back(fiber);
                }
            }

            self.lock.unlock();

            // 2. polling

            // 3. resched
            if let Some(mut f) = self.running.pop_back() {
                current_limbo.with(|limbo| {
                    limbo.borrow_mut().switch(&mut *f);
                });
            }

            // 4. bury the dead
            while let Some(_) = self.dying.pop_back() {
            }
        }
    }


    // TODO steal task
}
