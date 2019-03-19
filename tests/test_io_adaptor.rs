extern crate sleepyhead;

use sleepyhead::errno;
use sleepyhead::io::adaptor::Adaptor;
use std::io::{self, Read, Write};
use std::mem;

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

#[test]
fn test_io_adaptor_send_recv_pod_obj() {
    #[derive(Copy, Clone)]
    struct Foo {
        a: u32,
        b: char,
    }

    let foo = Foo {
        a: 0xdeadbeef,
        b: 'a',
    };

    let mut a = Adaptor::new();
    assert_eq!(a.send(foo).unwrap(), mem::size_of_val(&foo));

    let foo2: Foo = a.recv().unwrap();
    assert_eq!(foo2.a, foo.a);
    assert_eq!(foo2.b, foo.b);
}
