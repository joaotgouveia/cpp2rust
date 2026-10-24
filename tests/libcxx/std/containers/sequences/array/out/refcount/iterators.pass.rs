extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct NoDefault {}
impl NoDefault {
    pub fn new(mut _a0: ()) -> Self {
        Self {}
    }
}
pub fn check_noexcept_0(c: Ptr<()>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let cc: Ptr<()> = (c).clone();
    &(*cc.upgrade().deref());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn check_noexcept_1(c: Ptr<()>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let cc: Ptr<()> = (c).clone();
    &(*cc.upgrade().deref());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn check_noexcept_2(c: Ptr<()>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let cc: Ptr<()> = (c).clone();
    &(*cc.upgrade().deref());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn check_noexcept_3(c: Ptr<()>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let cc: Ptr<()> = (c).clone();
    &(*cc.upgrade().deref());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn check_noexcept_4(c: Ptr<()>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let cc: Ptr<()> = (c).clone();
    &(*cc.upgrade().deref());
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub fn tests_5() -> () {
    {
        let array: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_noexcept_0(array.as_pointer()) });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
    }
    {
        let array: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_noexcept_1(array.as_pointer()) });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
    }
    {
        let array: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_noexcept_1(array.as_pointer()) });
        let mut i: () = unimplemented!();
        let mut j: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let array: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_noexcept_2(array.as_pointer()) });
        let mut i: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        i.write(99_());
        unimplemented!();
    }
    {
        let array: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_noexcept_3(array.as_pointer()) });
        let mut i: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        i.write(99_());
        unimplemented!();
        unimplemented!();
    }
    {
        let array: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        ({ check_noexcept_4(array.as_pointer()) });
        let mut i: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        i.write(5.5_f64);
        unimplemented!();
        unimplemented!();
    }
    {
        let array: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut ib: () = unimplemented!();
        let mut ie: () = unimplemented!();
        unimplemented!();
    }
    {
        {
            let mut ii1: () = unimplemented!();
            let mut ii2: () = unimplemented!();
            let mut ii4: () = (ii1).clone();
            let mut cii: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ check_noexcept_0(c.as_pointer()) });
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
            unimplemented!();
            unimplemented!();
            let r1: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(&(ii1), &(ii2))));
            unimplemented!();
            let r2: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(&(cii), &(ii2))));
            unimplemented!();
        }
        {
            let mut ii1: () = unimplemented!();
            let mut ii2: () = unimplemented!();
            let mut ii4: () = (ii1).clone();
            let mut cii: () = unimplemented!();
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
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
            ({ check_noexcept_1(c.as_pointer()) });
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
            unimplemented!();
            unimplemented!();
            let r1: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(&(ii1), &(ii2))));
            unimplemented!();
            let r2: Value<()> = Rc::new(RefCell::new(std::cmp::Ord::cmp(&(cii), &(ii2))));
            unimplemented!();
        }
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
    ({ tests_5() });
    const _: () = assert!(true, "tests()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
