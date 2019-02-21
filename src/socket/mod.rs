use crate::aux;
use crate::Result;
use libc;
use libc::c_int;
use std::os::unix::io::RawFd;

mod addr;

pub use self::addr::AddressFamily;

#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum SockType {
    Stream = libc::SOCK_STREAM,
    Datagram = libc::SOCK_DGRAM,
    SeqPacket = libc::SOCK_SEQPACKET,
    ReliableDatagram = libc::SOCK_RDM,
}

#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum SockFlag {
    Nothing = 0,
    Nonblock = libc::SOCK_NONBLOCK,
    Cloexec = libc::SOCK_CLOEXEC,
}

pub fn socketpair(domain: AddressFamily, ty: SockType, flags: SockFlag) -> Result<(RawFd, RawFd)> {
    let ty = (ty as c_int) | (flags as c_int);

    let mut fds = [-1, -1];
    let result = unsafe { libc::socketpair(domain as c_int, ty, 0, fds.as_mut_ptr()) };
    aux::cvt(result)?;

    Ok((fds[0], fds[1]))
}

pub fn listen(fd: RawFd, backlog: usize) -> Result<()> {
    let result = unsafe { libc::listen(fd, backlog as c_int) };
    aux::cvt(result).map(|_| ())
}
