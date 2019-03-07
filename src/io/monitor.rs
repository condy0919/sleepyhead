use crate::aux;
use crate::ev::adaptor;
use libc;
use std::os::unix::io::RawFd;

pub struct Monitor {
    capacity: usize,
    epfd: RawFd,
    evext_chan: adaptor::Adaptor,
    // TODO event table
    // TODO timer heap
    timeout: i32,
    // TODO io extension point
    epevents: Vec<libc::epoll_event>,
}

impl Drop for Monitor {
    #[inline]
    fn drop(&mut self) {
        aux::close(self.epfd).unwrap();
    }
}

impl Monitor {
    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        let epfd = unsafe { libc::epoll_create1(libc::EPOLL_CLOEXEC) };
        if epfd == -1 {
            panic!("epoll_create1");
        }

        Monitor {
            capacity: cap,
            epfd: epfd,
            evext_chan: adaptor::Adaptor::new(),
            timeout: -1,
            epevents: Vec::with_capacity(cap),
        }
    }

    pub fn poll(&mut self) {
        // TODO PREPOLL phase
        let nfds = unsafe {
            libc::epoll_wait(
                self.epfd,
                self.epevents.as_ptr() as *mut _,
                self.capacity as i32,
                self.timeout,
            )
        };

        // TODO pretimed 队列 aka 快速请求队列

        if nfds == -1 {
            return;
        }


        // TODO PREIO
        for i in 0..nfds {
            let fd = self.epevents[i as usize].u64;

        }
    }
}
