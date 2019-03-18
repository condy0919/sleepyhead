use crate::aux;
use crate::fdflag::{FdFlag, Flag};
use crate::io::adaptor;
use libc;
use std::ops::BitOrAssign;
use std::os::unix::io::RawFd;

#[derive(Clone, Copy)]
pub enum FdAttribute {
    Empty = 0x0,
    EventChannel = 0x1,
}

impl BitOrAssign for FdAttribute {
    #[inline]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = FdAttribute::from((*self as u32) | (rhs as u32));
    }
}

impl From<u32> for FdAttribute {
    #[inline]
    fn from(opt: u32) -> Self {
        match opt {
            0x0 => FdAttribute::Empty,
            0x1 => FdAttribute::EventChannel,
            _ => panic!("unknown fd attribute"),
        }
    }
}

impl Into<u32> for FdAttribute {
    #[inline]
    fn into(self) -> u32 {
        match self {
            FdAttribute::Empty => 0x0,
            FdAttribute::EventChannel => 0x1,
        }
    }
}

impl Flag for FdAttribute {}

pub struct Monitor {
    capacity: usize,
    epfd: RawFd,
    evext_chan: adaptor::Adaptor,
    // TODO event table
    // TODO timer heap
    timeout: i32,
    // TODO io extension point
    epevs: Vec<libc::epoll_event>,
}

impl Drop for Monitor {
    #[inline]
    fn drop(&mut self) {
        aux::close(self.epfd).unwrap();
    }
}

impl Monitor {
    #[inline]
    pub fn with_size(sz: usize) -> Self {
        let epfd = unsafe { libc::epoll_create1(libc::EPOLL_CLOEXEC) };
        if epfd == -1 {
            panic!("epoll_create1");
        }

        let mut vec = Vec::new();
        vec.resize(sz, libc::epoll_event { events: 0, u64: 0 });

        Monitor {
            capacity: sz,
            epfd: epfd,
            evext_chan: adaptor::Adaptor::new(),
            timeout: -1,
            epevs: vec,
        }
    }

    pub fn register_ltrd(&mut self, fd: RawFd, attr: FdAttribute) {
        let mut ev = libc::epoll_event {
            events: libc::EPOLLIN as u32,
            u64: FdFlag::<FdAttribute>::new(fd, attr).into(),
        };

        unsafe { libc::epoll_ctl(self.epfd, libc::EPOLL_CTL_ADD, fd, &mut ev as *mut _); }
    }

    pub fn poll(&mut self) {
        // TODO PREPOLL phase
        let nfds = unsafe {
            libc::epoll_wait(
                self.epfd,
                self.epevs.as_ptr() as *mut _,
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
            let fdflag: FdFlag<FdAttribute> = self.epevs[i as usize].u64.into();
            match fdflag.get_flag() {
                FdAttribute::Empty => {}
                FdAttribute::EventChannel => {}
            };
        }
    }
}
