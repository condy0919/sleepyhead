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
    pub fn read_endpoint(&self) -> RawFd {
        self.fdpair.0
    }

    #[inline]
    pub fn write_endpoint(&self) -> RawFd {
        self.fdpair.1
    }

    #[inline]
    pub fn send<T: Sized + Copy + Send + Sync>(&mut self, obj: T) -> io::Result<usize> {
        aux::send(self.write_endpoint(), obj)
    }

    #[inline]
    pub fn recv<T: Sized + Copy + Send + Sync>(&mut self) -> io::Result<T> {
        aux::recv(self.read_endpoint())
    }
}

impl io::Read for Adaptor {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        aux::read(self.read_endpoint(), buf)
    }
}

impl io::Write for Adaptor {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        aux::write(self.write_endpoint(), buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
