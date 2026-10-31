extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_op_0(b1: u8, mut b2: u8) -> u8 {
    let b1: Value<u8> = Rc::new(RefCell::new(b1));
    const _: () = assert!(true, "noexcept(b1 |= b2)");;
    const _: () = assert!(true, "std::is_same_v<decltype(b1 |= b2), std::byte&>");;
    let ret: Ptr<u8> = {
        b1.as_pointer().write(b1.as_pointer().read() | b2);
        b1.as_pointer()
    };
    assert!(({ (ret).clone() } == { (b1.as_pointer()) }));
    return (ret.read());
}
pub fn test_1() -> bool {
    let mut b1: u8 = 1_u8;
    let mut b2: u8 = 2_u8;
    let mut b8: u8 = 8_u8;
    assert!((<i32>::cpp_to_integer(({ test_op_0(b1, b2,) })) == 3));
    assert!((<i32>::cpp_to_integer(({ test_op_0(b1, b8,) })) == 9));
    assert!((<i32>::cpp_to_integer(({ test_op_0(b2, b8,) })) == 10));
    assert!((<i32>::cpp_to_integer(({ test_op_0(b2, b1,) })) == 3));
    assert!((<i32>::cpp_to_integer(({ test_op_0(b8, b1,) })) == 9));
    assert!((<i32>::cpp_to_integer(({ test_op_0(b8, b2,) })) == 10));
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
    ({ test_1() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
