extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut fd: i32 = 0;
    let ssloc: Value<sockaddr_storage> = <Value<sockaddr_storage>>::default();
    let slen: Value<libc::socklen_t> = Rc::new(RefCell::new((128usize as libc::socklen_t)));
    assert!(
        (((match nix::sys::socket::getsockname::<nix::sys::socket::SockaddrStorage>(fd) {
            Ok(__ss) => {
                Sockaddr::encode(
                    &__ss,
                    &(ssloc.as_pointer()).reinterpret_cast::<sockaddr>(),
                    &(slen.as_pointer()),
                );
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == -1_i32) as i32)
            != 0)
    );
    let sin: Value<sockaddr_in> = <Value<sockaddr_in>>::default();
    let inlen: Value<libc::socklen_t> = Rc::new(RefCell::new((16usize as libc::socklen_t)));
    assert!(
        (((match nix::sys::socket::getsockname::<nix::sys::socket::SockaddrStorage>(fd) {
            Ok(__ss) => {
                Sockaddr::encode(
                    &__ss,
                    &(sin.as_pointer()).reinterpret_cast::<sockaddr>(),
                    &(inlen.as_pointer()),
                );
                0
            }
            Err(__e) => {
                libcc2rs::cpp2rust_errno().write(__e as i32);
                -1
            }
        } == -1_i32) as i32)
            != 0)
    );
    return 0;
}
pub fn __cpp2rust_init_globals() {}
