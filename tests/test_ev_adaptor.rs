extern crate sleepyhead;

use sleepyhead::errno;
use sleepyhead::ev::adaptor::Adaptor;
use std::io::{self, Read, Write};

#[test]
fn test_ev_adaptor_write_read() {
    let mut a = Adaptor::new();

    let xs = b"test";
    assert_eq!(a.write(xs).unwrap(), xs.len());

    let mut ys: [u8; 4] = Default::default();
    assert_eq!(a.read(&mut ys).unwrap(), xs.len());
}

#[test]
fn test_ev_adaptor_write_failed() {
    let mut a = Adaptor::default();

    a.write(b"test")
        .map(|_| panic!(""))
        .map_err(|e| {
            assert_eq!(e.kind(), io::ErrorKind::Other);

            e.get_ref().map(|inner_e| {
                assert_eq!(inner_e.description(), errno::Errno::EBADF.desc());
            });
        })
        .unwrap_err();
}
