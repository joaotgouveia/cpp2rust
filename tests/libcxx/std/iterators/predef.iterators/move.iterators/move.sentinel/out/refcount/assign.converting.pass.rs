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
pub struct NonAssignable {}
const _: () = assert!(true, "std::semiregular<NonAssignable>");
const _: () = assert!(true, "std::is_assignable_v<NonAssignable, int>");
const _: () = assert!(!(false), "!std::assignable_from<NonAssignable, int>");
pub fn test_0() -> () {
    {
        let m: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        let m2: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(
            true,
            "std::is_assignable_v<std::move_sentinel<int>, std::move_sentinel<long>>"
        );;
        const _: () = assert!(
            true,
            "!std::is_assignable_v<std::move_sentinel<int*>, std::move_sentinel<const int*>>"
        );;
        const _: () = assert!(
            true,
            "std::is_assignable_v<std::move_sentinel<const int*>, std::move_sentinel<int*>>"
        );;
        const _: () = assert!(
            true,
            "!std::is_assignable_v<std::move_sentinel<NonAssignable>, std::move_sentinel<int>>"
        );;
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
    ({ test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub trait NonAssignableImpl {
    fn operator_assign_1(&self, mut i: ()) -> Ptr<NonAssignable> {
        unimplemented!()
    }
}
impl NonAssignableImpl for Ptr<NonAssignable> {}
pub fn __cpp2rust_init_globals() {}
