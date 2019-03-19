use crate::fiber;
use crate::util::FnBox;
use crate::io::{adaptor, monitor};
use crate::util::spinlock;
use std::collections::VecDeque;


pub struct Scheduler<'m, 'f, 'c> {
    max_n_fiber: usize,
    iomon: &'m mut monitor::Monitor,
    running: VecDeque<fiber::Fiber<'f>>,
    blocking: VecDeque<fiber::Fiber<'f>>,
    dying: VecDeque<fiber::Fiber<'f>>,
    lock: spinlock::SpinLock,
    reqs: Vec<Box<FnBox + 'c>>,
    evch: adaptor::Adaptor,
    // TODO io extension points
}

impl<'m, 'f, 'c> Scheduler<'m, 'f, 'c> {
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

    // TODO post a req
    pub fn spawn<F: FnOnce() + Send + 'c>(&mut self, f: F) -> bool {
        if self.lock.try_lock() {
            self.reqs.push(Box::new(f));
            self.lock.unlock();
        } else {
            let b = Box::new(f);
        }

        true
    }

    // TODO mainloop
    // TODO steal task
}
