use std::alloc::{alloc, dealloc, Layout};

pub struct Stack {
    base: *mut u8,
    layout: Layout,
}

impl Stack {
    pub fn with_pages(n: usize) -> Option<Stack> {
        const PAGE_SIZE: usize = 4096;
        const STACK_ALIGNMENT_SIZE: usize = 16;

        let layout =
            unsafe { Layout::from_size_align_unchecked(PAGE_SIZE * n, STACK_ALIGNMENT_SIZE) };

        let mem = unsafe { alloc(layout) };

        if mem.is_null() {
            None
        } else {
            Some(Stack {
                base: mem,
                layout: layout,
            })
        }
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.base, self.layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_size_too_large() {
        let stack = Stack::with_pages(usize::max_value() >> 32);
        assert!(stack.is_none());
    }

    #[test]
    fn test_stack_size_normal() {
        let stack = Stack::with_pages(64);
        assert!(stack.is_some());
    }
}
