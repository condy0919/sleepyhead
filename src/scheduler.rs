use crate::fiber;
use crate::io::{adaptor, monitor};
use crate::util::spinlock;
use std::collections::VecDeque;

pub struct Scheduler<'m, 'f> {
    max_n_fiber: usize,
    iomon: &'m mut monitor::Monitor,
    running: VecDeque<fiber::Fiber<'f>>,
    blocking: VecDeque<fiber::Fiber<'f>>,
    dying: VecDeque<fiber::Fiber<'f>>,
    lock: spinlock::SpinLock,
    reqs: Vec<Box<FnOnce()>>,
    evch: adaptor::Adaptor,
    // TODO io extension points
}

impl<'m, 'f> Scheduler<'m, 'f> {
    pub fn new(max_n_fiber: usize, iomon: &'m mut monitor::Monitor) -> Self {
        let evch = adaptor::Adaptor::with_nonblock();
        iomon.register_ltrd(evch.read_end(), monitor::FdAttribute::EventChannel);

        Scheduler {
            max_n_fiber,
            iomon,
            running: VecDeque::new(),
            blocking: VecDeque::new(),
            dying: VecDeque::new(),
            lock: spinlock::SpinLock::new(),
            reqs: Vec::new(),
            evch: evch,
        }
    }

    // TODO post a req
    // TODO mainloop
    // TODO steal task
}
