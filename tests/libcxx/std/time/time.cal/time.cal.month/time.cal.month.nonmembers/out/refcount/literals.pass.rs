extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_std_chrono_month__const_std_chrono_month_ {}
impl CppMove for test_macros_detail_is_same_const_std_chrono_month__const_std_chrono_month_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
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
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "std::chrono::January   == std::chrono::month(1)");;
    const _: () = assert!(true, "std::chrono::February  == std::chrono::month(2)");;
    const _: () = assert!(true, "std::chrono::March     == std::chrono::month(3)");;
    const _: () = assert!(true, "std::chrono::April     == std::chrono::month(4)");;
    const _: () = assert!(true, "std::chrono::May       == std::chrono::month(5)");;
    const _: () = assert!(true, "std::chrono::June      == std::chrono::month(6)");;
    const _: () = assert!(true, "std::chrono::July      == std::chrono::month(7)");;
    const _: () = assert!(true, "std::chrono::August    == std::chrono::month(8)");;
    const _: () = assert!(true, "std::chrono::September == std::chrono::month(9)");;
    const _: () = assert!(true, "std::chrono::October   == std::chrono::month(10)");;
    const _: () = assert!(true, "std::chrono::November  == std::chrono::month(11)");;
    const _: () = assert!(true, "std::chrono::December  == std::chrono::month(12)");;
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!(unimplemented!());
    assert!((unimplemented!() == 1_u32));
    assert!((unimplemented!() == 2_u32));
    assert!((unimplemented!() == 3_u32));
    assert!((unimplemented!() == 4_u32));
    assert!((unimplemented!() == 5_u32));
    assert!((unimplemented!() == 6_u32));
    assert!((unimplemented!() == 7_u32));
    assert!((unimplemented!() == 8_u32));
    assert!((unimplemented!() == 9_u32));
    assert!((unimplemented!() == 10_u32));
    assert!((unimplemented!() == 11_u32));
    assert!((unimplemented!() == 12_u32));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
