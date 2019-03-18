use std::sync::atomic::{spin_loop_hint, AtomicBool, Ordering};

pub struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    pub fn new() -> Self {
        SpinLock {
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&mut self) {
        while self.locked.load(Ordering::Relaxed) || !self.try_lock() {
            spin_loop_hint();
        }
    }

    pub fn unlock(&mut self) {
        self.locked.store(false, Ordering::Relaxed);
    }

    pub fn try_lock(&mut self) -> bool {
        self.locked
            .compare_exchange_weak(false, true, Ordering::SeqCst, Ordering::Relaxed)
            .is_ok()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinlock_lock() {
        let mut lock = SpinLock::new();
        lock.lock();
        assert!(!lock.try_lock());
    }
}
