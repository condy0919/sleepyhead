extern crate sleepyhead;

use sleepyhead::errno;
use sleepyhead::io::adaptor::Adaptor;
use std::io::{self, Read, Write};

#[test]
fn test_io_adaptor_write_read() {
    let mut a = Adaptor::new();

    let xs = b"test";
    assert_eq!(a.write(xs).unwrap(), xs.len());

    let mut ys: [u8; 4] = Default::default();
    assert_eq!(a.read(&mut ys).unwrap(), xs.len());
}

#[test]
fn test_io_adaptor_write_failed() {
    let mut a = Adaptor::default();

    a.write(b"test")
        .map(|_| panic!(""))
        .map_err(|e| {
            assert_eq!(e.kind(), io::ErrorKind::Other);
            assert_eq!(e.raw_os_error().unwrap(), errno::Errno::EBADF as i32);
        })
        .unwrap_err();
}
