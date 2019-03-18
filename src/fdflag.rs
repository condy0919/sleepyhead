use std::ops::BitOrAssign;
use std::os::unix::io::RawFd;

// opt
// 1. EXISTS ERROR 用于在注册时
// 2. epoll 触发时，用来判断是 EPOLL 事件，还是通过 channel 传递的要创建 uthread 的信息

pub trait Flag: From<u32> + Into<u32> + Clone + Copy + BitOrAssign {}

pub struct FdFlag<T: Flag> {
    fd: RawFd,
    flag: T,
}

impl<T: Flag> From<u64> for FdFlag<T> {
    #[inline]
    fn from(fdflag: u64) -> Self {
        let fd = (fdflag & 0x00000000ffffffff) as RawFd;
        let flag = T::from((fdflag >> 32) as u32);

        FdFlag { fd, flag }
    }
}

impl<T: Flag> Into<u64> for FdFlag<T> {
    #[inline]
    fn into(self) -> u64 {
        (u64::from(self.flag.into()) << 32) | u64::from(self.fd as u32)
    }
}

impl<T: Flag> FdFlag<T> {
    #[inline]
    pub fn new(fd: RawFd, flag: T) -> Self {
        FdFlag { fd, flag }
    }

    #[inline]
    pub fn get_fd(&self) -> RawFd {
        self.fd
    }

    #[inline]
    pub fn get_flag(&self) -> T {
        self.flag
    }

    #[inline]
    pub fn or_flag(&mut self, o: T) -> &mut Self {
        self.flag |= o;
        self
    }
}
