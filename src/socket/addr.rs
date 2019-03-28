use libc;

#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum AddressFamily {
    Unix = libc::AF_UNIX,
    Inet = libc::AF_INET,
    Inet6 = libc::AF_INET6,
    Unspec = libc::AF_UNSPEC,
}

impl From<i32> for AddressFamily {
    fn from(family: i32) -> Self {
        match family {
            libc::AF_UNIX => AddressFamily::Unix,
            libc::AF_INET => AddressFamily::Inet,
            libc::AF_INET6 => AddressFamily::Inet6,
            libc::AF_UNSPEC => AddressFamily::Unspec,
            _ => panic!("unknown address family"),
        }
    }
}
