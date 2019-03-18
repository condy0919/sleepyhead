use crate::aux;
use crate::socket;
use std::os::unix::io::RawFd;
use std::io;

pub struct Adaptor {
    fdpair: (RawFd, RawFd),
}

impl Default for Adaptor {
    fn default() -> Self {
        Adaptor {
            fdpair: (-1, -1),
        }
    }
}

impl Drop for Adaptor {
    #[inline]
    fn drop(&mut self) {
        aux::close(self.fdpair.0).unwrap_or_else(|_| ());
        aux::close(self.fdpair.1).unwrap_or_else(|_| ());
    }
}

impl Adaptor {
    #[inline]
    pub fn new() -> Self {
        let result = socket::socketpair(socket::AddressFamily::Unix,
                                        socket::SockType::Stream,
                                        socket::SockFlag::Nothing);
        Adaptor {
            fdpair: result.unwrap_or((-1, -1)),
        }
    }

    #[inline]
    pub fn with_nonblock() -> Self {
        let result = socket::socketpair(socket::AddressFamily::Unix,
                                        socket::SockType::Stream,
                                        socket::SockFlag::Nonblock);
        Adaptor {
            fdpair: result.unwrap_or((-1, -1)),
        }
    }

    #[inline]
    pub fn read_end(&self) -> RawFd {
        self.fdpair.0
    }

    #[inline]
    pub fn write_end(&self) -> RawFd {
        self.fdpair.1
    }
}

impl io::Read for Adaptor {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        aux::read(self.read_end(), buf)
    }
}

impl io::Write for Adaptor {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        aux::write(self.write_end(), buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
