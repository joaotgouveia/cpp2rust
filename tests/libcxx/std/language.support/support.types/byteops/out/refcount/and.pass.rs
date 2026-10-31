extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
const _: () = assert!(true, "noexcept(std::byte{{}} & std::byte{{}})");
const _: () = assert!(
    true,
    "std::is_same_v<decltype(std::byte{{}} & std::byte{{}}), std::byte>"
);
pub fn test_0() -> bool {
    let mut b1: u8 = 1_u8;
    let mut b8: u8 = 8_u8;
    let mut b9: u8 = 9_u8;
    assert!((<i32>::cpp_to_integer(b1 & b8) == 0));
    assert!((<i32>::cpp_to_integer(b1 & b9) == 1));
    assert!((<i32>::cpp_to_integer(b8 & b9) == 8));
    assert!((<i32>::cpp_to_integer(b8 & b1) == 0));
    assert!((<i32>::cpp_to_integer(b9 & b1) == 1));
    assert!((<i32>::cpp_to_integer(b9 & b8) == 8));
    return true;
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
