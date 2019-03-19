pub mod util;
pub mod errno;
pub mod socket;
pub mod io;
pub mod fiber;
pub mod worker;
pub mod scheduler;

mod fdflag;

mod aux {
    use libc;
    use libc::size_t;
    use std::io;
    use std::mem;
    use std::os::unix::io::RawFd;
    use std::slice;

    #[doc(hidden)]
    pub trait IsMinusOne {
        fn is_minus_one(&self) -> bool;
    }

    macro_rules! impl_is_minus_one {
        ($($t:ident)*) => {
            $(impl IsMinusOne for $t {
                fn is_minus_one(&self) -> bool {
                    *self == -1
                }
            })*
        }
    }

    impl_is_minus_one! { i8 i16 i32 i64 isize }


    #[inline]
    pub fn cvt<T: IsMinusOne>(t: T) -> io::Result<T> {
        if t.is_minus_one() {
            Err(io::Error::last_os_error())
        } else {
            Ok(t)
        }
    }

    #[inline]
    pub fn cvt_r<T, F>(mut f: F) -> io::Result<T>
        where T: IsMinusOne,
              F: FnMut() -> T,
    {
        loop {
            match cvt(f()) {
                Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {},
                other => return other,
            }
        }
    }

    #[inline]
    pub fn read(fd: RawFd, buf: &mut [u8]) -> io::Result<usize> {
        let result = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut _, buf.len() as size_t) };
        cvt(result).map(|r| r as usize)
    }

    #[inline]
    pub fn write(fd: RawFd, buf: &[u8]) -> io::Result<usize> {
        let result = unsafe { libc::write(fd, buf.as_ptr() as *const _, buf.len() as size_t) };
        cvt(result).map(|r| r as usize)
    }

    #[inline]
    pub fn close(fd: RawFd) -> io::Result<()> {
        let result = unsafe { libc::close(fd) };
        cvt(result).map(|_| ())
    }

    #[inline]
    pub fn send<T: Sized + Sync + Send + Copy>(fd: RawFd, obj: T) -> io::Result<usize> {
        let buf: &[u8] = unsafe {
            slice::from_raw_parts(&obj as *const _ as *const _, mem::size_of::<T>())
        };
        write(fd, buf)
    }

    #[inline]
    pub fn recv<T: Sized + Sync + Send + Copy>(fd: RawFd) -> io::Result<T> {
        let mut obj: T = unsafe { mem::uninitialized() };
        let buf: &mut [u8] = unsafe {
            slice::from_raw_parts_mut(&mut obj as *mut _ as *mut _, mem::size_of::<T>())
        };

        read(fd, buf).map(|_| obj)
    }
}
