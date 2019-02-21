use libc;
use std::error;
use std::fmt;
use std::io;
use std::mem;

extern "C" {
    #[link_name = "__errno_location"]
    fn errno_location() -> *mut libc::c_int;
}

#[inline]
pub fn errno() -> i32 {
    unsafe { (*errno_location()) as i32 }
}

#[inline]
pub fn set_errno(e: i32) {
    unsafe { *errno_location() = e as libc::c_int }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum Errno {
    SUCCESS         = 0,
    EPERM           = libc::EPERM,
    ENOENT          = libc::ENOENT,
    ESRCH           = libc::ESRCH,
    EINTR           = libc::EINTR,
    EIO             = libc::EIO,
    ENXIO           = libc::ENXIO,
    E2BIG           = libc::E2BIG,
    ENOEXEC         = libc::ENOEXEC,
    EBADF           = libc::EBADF,
    ECHILD          = libc::ECHILD,
    EAGAIN          = libc::EAGAIN,
    ENOMEM          = libc::ENOMEM,
    EACCES          = libc::EACCES,
    EFAULT          = libc::EFAULT,
    ENOTBLK         = libc::ENOTBLK,
    EBUSY           = libc::EBUSY,
    EEXIST          = libc::EEXIST,
    EXDEV           = libc::EXDEV,
    ENODEV          = libc::ENODEV,
    ENOTDIR         = libc::ENOTDIR,
    EISDIR          = libc::EISDIR,
    EINVAL          = libc::EINVAL,
    ENFILE          = libc::ENFILE,
    EMFILE          = libc::EMFILE,
    ENOTTY          = libc::ENOTTY,
    ETXTBSY         = libc::ETXTBSY,
    EFBIG           = libc::EFBIG,
    ENOSPC          = libc::ENOSPC,
    ESPIPE          = libc::ESPIPE,
    EROFS           = libc::EROFS,
    EMLINK          = libc::EMLINK,
    EPIPE           = libc::EPIPE,
    EDOM            = libc::EDOM,
    ERANGE          = libc::ERANGE,
    EDEADLK         = libc::EDEADLK,
    ENAMETOOLONG    = libc::ENAMETOOLONG,
    ENOLCK          = libc::ENOLCK,
    ENOSYS          = libc::ENOSYS,
    ENOTEMPTY       = libc::ENOTEMPTY,
    ELOOP           = libc::ELOOP,
    //EWOULDBLOCK   = libc::EWOULDBLOCK,
    ENOMSG          = libc::ENOMSG,
    EIDRM           = libc::EIDRM,
    ECHRNG          = libc::ECHRNG,
    EL2NSYNC        = libc::EL2NSYNC,
    EL3HLT          = libc::EL3HLT,
    EL3RST          = libc::EL3RST,
    ELNRNG          = libc::ELNRNG,
    EUNATCH         = libc::EUNATCH,
    ENOCSI          = libc::ENOCSI,
    EL2HLT          = libc::EL2HLT,
    EBADE           = libc::EBADE,
    EBADR           = libc::EBADR,
    EXFULL          = libc::EXFULL,
    ENOANO          = libc::ENOANO,
    EBADRQC         = libc::EBADRQC,
    EBADSLT         = libc::EBADSLT,
    //EDEADLOCK     = libc::EDEADLOCK,
    EBFONT          = libc::EBFONT,
    ENOSTR          = libc::ENOSTR,
    ENODATA         = libc::ENODATA,
    ETIME           = libc::ETIME,
    ENOSR           = libc::ENOSR,
    ENONET          = libc::ENONET,
    ENOPKG          = libc::ENOPKG,
    EREMOTE         = libc::EREMOTE,
    ENOLINK         = libc::ENOLINK,
    EADV            = libc::EADV,
    ESRMNT          = libc::ESRMNT,
    ECOMM           = libc::ECOMM,
    EPROTO          = libc::EPROTO,
    EMULTIHOP       = libc::EMULTIHOP,
    EDOTDOT         = libc::EDOTDOT,
    EBADMSG         = libc::EBADMSG,
    EOVERFLOW       = libc::EOVERFLOW,
    ENOTUNIQ        = libc::ENOTUNIQ,
    EBADFD          = libc::EBADFD,
    EREMCHG         = libc::EREMCHG,
    ELIBACC         = libc::ELIBACC,
    ELIBBAD         = libc::ELIBBAD,
    ELIBSCN         = libc::ELIBSCN,
    ELIBMAX         = libc::ELIBMAX,
    ELIBEXEC        = libc::ELIBEXEC,
    EILSEQ          = libc::EILSEQ,
    ERESTART        = libc::ERESTART,
    ESTRPIPE        = libc::ESTRPIPE,
    EUSERS          = libc::EUSERS,
    ENOTSOCK        = libc::ENOTSOCK,
    EDESTADDRREQ    = libc::EDESTADDRREQ,
    EMSGSIZE        = libc::EMSGSIZE,
    EPROTOTYPE      = libc::EPROTOTYPE,
    ENOPROTOOPT     = libc::ENOPROTOOPT,
    EPROTONOSUPPORT = libc::EPROTONOSUPPORT,
    ESOCKTNOSUPPORT = libc::ESOCKTNOSUPPORT,
    EOPNOTSUPP      = libc::EOPNOTSUPP,
    EPFNOSUPPORT    = libc::EPFNOSUPPORT,
    EAFNOSUPPORT    = libc::EAFNOSUPPORT,
    EADDRINUSE      = libc::EADDRINUSE,
    EADDRNOTAVAIL   = libc::EADDRNOTAVAIL,
    ENETDOWN        = libc::ENETDOWN,
    ENETUNREACH     = libc::ENETUNREACH,
    ENETRESET       = libc::ENETRESET,
    ECONNABORTED    = libc::ECONNABORTED,
    ECONNRESET      = libc::ECONNRESET,
    ENOBUFS         = libc::ENOBUFS,
    EISCONN         = libc::EISCONN,
    ENOTCONN        = libc::ENOTCONN,
    ESHUTDOWN       = libc::ESHUTDOWN,
    ETOOMANYREFS    = libc::ETOOMANYREFS,
    ETIMEDOUT       = libc::ETIMEDOUT,
    ECONNREFUSED    = libc::ECONNREFUSED,
    EHOSTDOWN       = libc::EHOSTDOWN,
    EHOSTUNREACH    = libc::EHOSTUNREACH,
    EALREADY        = libc::EALREADY,
    EINPROGRESS     = libc::EINPROGRESS,
    ESTALE          = libc::ESTALE,
    EUCLEAN         = libc::EUCLEAN,
    ENOTNAM         = libc::ENOTNAM,
    ENAVAIL         = libc::ENAVAIL,
    EISNAM          = libc::EISNAM,
    EREMOTEIO       = libc::EREMOTEIO,
    EDQUOT          = libc::EDQUOT,
    ENOMEDIUM       = libc::ENOMEDIUM,
    EMEDIUMTYPE     = libc::EMEDIUMTYPE,
    ECANCELED       = libc::ECANCELED,
    ENOKEY          = libc::ENOKEY,
    EKEYEXPIRED     = libc::EKEYEXPIRED,
    EKEYREVOKED     = libc::EKEYREVOKED,
    EKEYREJECTED    = libc::EKEYREJECTED,
    EOWNERDEAD      = libc::EOWNERDEAD,
    ENOTRECOVERABLE = libc::ENOTRECOVERABLE,
    ERFKILL         = libc::ERFKILL,
    EHWPOISON       = libc::EHWPOISON,
    //ENOTSUP       = libc::ENOTSUP,
}

impl From<i32> for Errno {
    #[inline]
    fn from(e: i32) -> Self {
        unsafe { mem::transmute_copy(&e) }
    }
}

impl fmt::Display for Errno {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self, self.desc())
    }
}

impl error::Error for Errno {
    #[inline]
    fn description(&self) -> &str {
        self.desc()
    }
}

impl From<Errno> for io::Error {
    #[inline]
    fn from(e: Errno) -> Self {
        io::Error::new(io::ErrorKind::Other, e)
    }
}

impl Errno {
    #[inline]
    pub fn last() -> Self {
        errno().into()
    }

    #[inline]
    pub fn desc(self) -> &'static str {
        use Errno::*;

        match self {
            SUCCESS         => "Success",
            EPERM           => "Operation not permitted",
            ENOENT          => "No such file or directory",
            ESRCH           => "No such process",
            EINTR           => "Interrupted system call",
            EIO             => "Input/output error",
            ENXIO           => "No such device or address",
            E2BIG           => "Argument list too long",
            ENOEXEC         => "Exec format error",
            EBADF           => "Bad file descriptor",
            ECHILD          => "No child processes",
            EAGAIN          => "Resource temporarily unavailable",
            ENOMEM          => "Cannot allocate memory",
            EACCES          => "Permission denied",
            EFAULT          => "Bad address",
            ENOTBLK         => "Block device required",
            EBUSY           => "Device or resource busy",
            EEXIST          => "File exists",
            EXDEV           => "Invalid cross-device link",
            ENODEV          => "No such device",
            ENOTDIR         => "Not a directory",
            EISDIR          => "Is a directory",
            EINVAL          => "Invalid argument",
            ENFILE          => "Too many open files in system",
            EMFILE          => "Too many open files",
            ENOTTY          => "Inappropriate ioctl for device",
            ETXTBSY         => "Text file busy",
            EFBIG           => "File too large",
            ENOSPC          => "No space left on device",
            ESPIPE          => "Illegal seek",
            EROFS           => "Read-only file system",
            EMLINK          => "Too many links",
            EPIPE           => "Broken pipe",
            EDOM            => "Numerical argument out of domain",
            ERANGE          => "Numerical result out of range",
            EDEADLK         => "Resource deadlock avoided",
            ENAMETOOLONG    => "File name too long",
            ENOLCK          => "No locks available",
            ENOSYS          => "Function not implemented",
            ENOTEMPTY       => "Directory not empty",
            ELOOP           => "Too many levels of symbolic links",
            //EWOULDBLOCK   => "Resource temporarily unavailable",
            ENOMSG          => "No message of desired type",
            EIDRM           => "Identifier removed",
            ECHRNG          => "Channel number out of range",
            EL2NSYNC        => "Level 2 not synchronized",
            EL3HLT          => "Level 3 halted",
            EL3RST          => "Level 3 reset",
            ELNRNG          => "Link number out of range",
            EUNATCH         => "Protocol driver not attached",
            ENOCSI          => "No CSI structure available",
            EL2HLT          => "Level 2 halted",
            EBADE           => "Invalid exchange",
            EBADR           => "Invalid request descriptor",
            EXFULL          => "Exchange full",
            ENOANO          => "No anode",
            EBADRQC         => "Invalid request code",
            EBADSLT         => "Invalid slot",
            //EDEADLOCK     => "Resource deadlock avoided",
            EBFONT          => "Bad font file format",
            ENOSTR          => "Device not a stream",
            ENODATA         => "No data available",
            ETIME           => "Timer expired",
            ENOSR           => "Out of streams resources",
            ENONET          => "Machine is not on the network",
            ENOPKG          => "Package not installed",
            EREMOTE         => "Object is remote",
            ENOLINK         => "Link has been severed",
            EADV            => "Advertise error",
            ESRMNT          => "Srmount error",
            ECOMM           => "Communication error on send",
            EPROTO          => "Protocol error",
            EMULTIHOP       => "Multihop attempted",
            EDOTDOT         => "RFS specific error",
            EBADMSG         => "Bad message",
            EOVERFLOW       => "Value too large for defined data type",
            ENOTUNIQ        => "Name not unique on network",
            EBADFD          => "File descriptor in bad state",
            EREMCHG         => "Remote address changed",
            ELIBACC         => "Can not access a needed shared library",
            ELIBBAD         => "Accessing a corrupted shared library",
            ELIBSCN         => ".lib section in a.out corrupted",
            ELIBMAX         => "Attempting to link in too many shared libraries",
            ELIBEXEC        => "Cannot exec a shared library directly",
            EILSEQ          => "Invalid or incomplete multibyte or wide character",
            ERESTART        => "Interrupted system call should be restarted",
            ESTRPIPE        => "Streams pipe error",
            EUSERS          => "Too many users",
            ENOTSOCK        => "Socket operation on non-socket",
            EDESTADDRREQ    => "Destination address required",
            EMSGSIZE        => "Message too long",
            EPROTOTYPE      => "Protocol wrong type for socket",
            ENOPROTOOPT     => "Protocol not available",
            EPROTONOSUPPORT => "Protocol not supported",
            ESOCKTNOSUPPORT => "Socket type not supported",
            EOPNOTSUPP      => "Operation not supported",
            EPFNOSUPPORT    => "Protocol family not supported",
            EAFNOSUPPORT    => "Address family not supported by protocol",
            EADDRINUSE      => "Address already in use",
            EADDRNOTAVAIL   => "Cannot assign requested address",
            ENETDOWN        => "Network is down",
            ENETUNREACH     => "Network is unreachable",
            ENETRESET       => "Network dropped connection on reset",
            ECONNABORTED    => "Software caused connection abort",
            ECONNRESET      => "Connection reset by peer",
            ENOBUFS         => "No buffer space available",
            EISCONN         => "Transport endpoint is already connected",
            ENOTCONN        => "Transport endpoint is not connected",
            ESHUTDOWN       => "Cannot send after transport endpoint shutdown",
            ETOOMANYREFS    => "Too many references: cannot splice",
            ETIMEDOUT       => "Connection timed out",
            ECONNREFUSED    => "Connection refused",
            EHOSTDOWN       => "Host is down",
            EHOSTUNREACH    => "No route to host",
            EALREADY        => "Operation already in progress",
            EINPROGRESS     => "Operation now in progress",
            ESTALE          => "Stale file handle",
            EUCLEAN         => "Structure needs cleaning",
            ENOTNAM         => "Not a XENIX named type file",
            ENAVAIL         => "No XENIX semaphores available",
            EISNAM          => "Is a named type file",
            EREMOTEIO       => "Remote I/O error",
            EDQUOT          => "Disk quota exceeded",
            ENOMEDIUM       => "No medium found",
            EMEDIUMTYPE     => "Wrong medium type",
            ECANCELED       => "Operation canceled",
            ENOKEY          => "Required key not available",
            EKEYEXPIRED     => "Key has expired",
            EKEYREVOKED     => "Key has been revoked",
            EKEYREJECTED    => "Key was rejected by service",
            EOWNERDEAD      => "Owner died",
            ENOTRECOVERABLE => "State not recoverable",
            ERFKILL         => "Operation not possible due to RF-kill",
            EHWPOISON       => "Memory page has hardware error",
            //ENOTSUP       => "Operation not supported",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_errno() {
        assert_eq!(errno(), 0);
    }

    #[test]
    fn test_errno_transmute() {
        let e = Errno::from(Errno::EAGAIN);
        assert_eq!(e.desc(), Errno::EAGAIN.desc());
    }
}
