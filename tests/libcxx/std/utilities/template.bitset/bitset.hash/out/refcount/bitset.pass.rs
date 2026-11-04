extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_0() {
    const _: () = assert!(true, "");;
    let h: Value<std_hash_std_bitset_0__> =
        Rc::new(RefCell::new(<std_hash_std_bitset_0__>::default()));
    let bs: Value<std_bitset_0_> = Rc::new(RefCell::new(std_bitset_0_::new_1({ 0_u64 })));
    let mut result: usize = ({ (*h.borrow_mut())(bs.as_pointer()) });
    const _: () = assert!(true, "");;
    (&(result));
}
pub fn test_1() {
    const _: () = assert!(true, "");;
    let h: Value<std_hash_std_bitset_10__> =
        Rc::new(RefCell::new(<std_hash_std_bitset_10__>::default()));
    let bs: Value<std_bitset_10_> = Rc::new(RefCell::new(std_bitset_10_::new_1({ 10_u64 })));
    let mut result: usize = ({ (*h.borrow_mut())(bs.as_pointer()) });
    const _: () = assert!(true, "");;
    (&(result));
}
pub fn test_2() {
    const _: () = assert!(true, "");;
    let h: Value<std_hash_std_bitset_100__> =
        Rc::new(RefCell::new(<std_hash_std_bitset_100__>::default()));
    let bs: Value<std_bitset_100_> = Rc::new(RefCell::new(std_bitset_100_::new_1({ 100_u64 })));
    let mut result: usize = ({ (*h.borrow_mut())(bs.as_pointer()) });
    const _: () = assert!(true, "");;
    (&(result));
}
pub fn test_3() {
    const _: () = assert!(true, "");;
    let h: Value<std_hash_std_bitset_1000__> =
        Rc::new(RefCell::new(<std_hash_std_bitset_1000__>::default()));
    let bs: Value<std_bitset_1000_> = Rc::new(RefCell::new(std_bitset_1000_::new_1({ 1000_u64 })));
    let mut result: usize = ({ (*h.borrow_mut())(bs.as_pointer()) });
    const _: () = assert!(true, "");;
    (&(result));
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
    ({ test_1() });
    ({ test_2() });
    ({ test_3() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
