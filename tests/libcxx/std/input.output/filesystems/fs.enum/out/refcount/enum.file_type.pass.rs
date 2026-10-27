extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn ME_0(mut val: ()) -> () {
    return ((val) as ());
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    const _: () = assert!(true, "std::is_enum<E>::value");;
    const _: () = assert!(true, "!std::is_convertible<E, UT>::value");;
    const _: () = assert!(true, "std::is_same<UT, signed char>::value");;
    const _: () = assert!(
        true,
        "E::none == ME(0) &&\n          E::not_found == ME(-1) &&\n          E::regular == ME(1) &&\n          E::directory == ME(2) &&\n          E::symlink == ME(3) &&\n          E::block == ME(4) &&\n          E::character == ME(5) &&\n          E::fifo == ME(6) &&\n          E::socket == ME(7) &&\n          E::unknown == ME(8)"
    );;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
