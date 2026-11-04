extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn check_required_alignment_0() {
    let mut required_alignment: usize = required_alignment_1.with(|rc| *rc.borrow());
    assert!((required_alignment >= ::std::mem::align_of::<i32>()));
}
pub fn check_required_alignment_2() {
    let mut required_alignment: usize = required_alignment_3.with(|rc| *rc.borrow());
    assert!((required_alignment >= ::std::mem::align_of::<f32>()));
}
pub fn check_required_alignment_4() {
    let mut required_alignment: usize = required_alignment_5.with(|rc| *rc.borrow());
    assert!((required_alignment >= 8usize));
}
pub fn check_required_alignment_6() {
    let mut required_alignment: usize = required_alignment_7.with(|rc| *rc.borrow());
    assert!((required_alignment >= 1usize));
}
pub fn check_required_alignment_8() {
    let mut required_alignment: usize = required_alignment_9.with(|rc| *rc.borrow());
    assert!((required_alignment >= 4usize));
}
pub fn test_10() -> bool {
    ({ check_required_alignment_0() });
    ({ check_required_alignment_2() });
    ({ check_required_alignment_4() });
    ({ check_required_alignment_6() });
    ({ check_required_alignment_8() });
    return true;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Empty_11 {}
impl CppMove for Empty_11 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Trivial_12 {
    #[offset(0)]
    pub a: i32,
}
impl CppMove for Trivial_12 {
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
    ({ test_10() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
