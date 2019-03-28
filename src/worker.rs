use std::sync::{Arc, Barrier};
use std::thread;

pub struct Worker {
    // TODO scheduler
    // TODO iomonitor
    handle: thread::JoinHandle<()>,
}

pub struct WorkerPool {
    size: usize,
    workers: Vec<Worker>,
    barrier: Arc<Barrier>,
}

impl WorkerPool {
    pub fn with_capacity(sz: usize) -> Self {
        WorkerPool {
            size: sz,
            workers: Vec::with_capacity(sz),
            barrier: Arc::new(Barrier::new(sz)),
        }
    }

    pub fn launch(&mut self) {
        for _ in 0..self.size {
            self.workers.push(Worker::new(self.barrier.clone()));
        }
    }

    // TODO get idx's scheduler
    //

    pub fn stop(self) {
        for w in self.workers {
            w.stop();
        }
    }
}

impl Worker {
    pub fn new(barrier: Arc<Barrier>) -> Self {
        Worker {
            handle: thread::spawn(move || {
                barrier.wait();
                // TODO schedloop
            }),
        }
    }

    pub fn stop(self) {
        self.handle.join().unwrap();
    }
}
