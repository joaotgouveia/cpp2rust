extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_constexpr_0() -> () {
    let mut v: () = 12_();
    if (12_() != unimplemented!()) || (v != 23_()) {
        return false;
    }
    if (23_() != unimplemented!()) || (v != 67_()) {
        return false;
    }
    if (67_() != unimplemented!()) || (v != 0_()) {
        return false;
    }
    return true;
}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestNoexcept_true__true_ {}
impl TestNoexcept_true__true_ {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestNoexcept_true__false_ {}
impl TestNoexcept_true__false_ {}
#[derive(Record, ByteRepr, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestNoexcept_false__true_ {}
impl TestNoexcept_false__true_ {}
pub fn test_noexcept_1() -> () {
    {
        let mut x: () = 42_();
        const _: () = assert!(true, "");;
    }
    {
        let x: Value<TestNoexcept_true__true_> =
            Rc::new(RefCell::new(<TestNoexcept_true__true_>::default()));
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        let x: Value<TestNoexcept_true__false_> =
            Rc::new(RefCell::new(<TestNoexcept_true__false_>::default()));
        const _: () = assert!(true, "");;
    }
    {
        let x: Value<TestNoexcept_false__true_> =
            Rc::new(RefCell::new(<TestNoexcept_false__true_>::default()));
        const _: () = assert!(true, "");;
    }
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    {
        let mut v: () = 12_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut b: () = false;
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s1: () = unimplemented!();
        let mut s2: () = unimplemented!();
        let mut s3: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    const _: () = assert!(true, "test_constexpr()");;
    const _: () = assert!(true, "test_noexcept()");;
    return 0_();
}
pub trait TestNoexcept_false__true_Impl {
    fn copy_assign(&self, _a0: Ptr<TestNoexcept_false__true_>) -> Ptr<TestNoexcept_false__true_> {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<TestNoexcept_false__true_>) -> Ptr<TestNoexcept_false__true_> {
        unimplemented!()
    }
}
impl TestNoexcept_false__true_Impl for Ptr<TestNoexcept_false__true_> {}
pub trait TestNoexcept_true__false_Impl {
    fn copy_assign(&self, _a0: Ptr<TestNoexcept_true__false_>) -> Ptr<TestNoexcept_true__false_> {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<TestNoexcept_true__false_>) -> Ptr<TestNoexcept_true__false_> {
        unimplemented!()
    }
}
impl TestNoexcept_true__false_Impl for Ptr<TestNoexcept_true__false_> {}
pub trait TestNoexcept_true__true_Impl {
    fn copy_assign(&self, _a0: Ptr<TestNoexcept_true__true_>) -> Ptr<TestNoexcept_true__true_> {
        unimplemented!()
    }
    fn move_assign(&self, _a0: Ptr<TestNoexcept_true__true_>) -> Ptr<TestNoexcept_true__true_> {
        unimplemented!()
    }
}
impl TestNoexcept_true__true_Impl for Ptr<TestNoexcept_true__true_> {}
pub fn __cpp2rust_init_globals() {}
