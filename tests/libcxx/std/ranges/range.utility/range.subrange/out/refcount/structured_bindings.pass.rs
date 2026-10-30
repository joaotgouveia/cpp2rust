extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_std_unreachable_sentinel_t__std_unreachable_sentinel_t_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
pub fn test_sized_subrange_2() {
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_()])));
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let __decomp_3: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut first: () = unimplemented!();
        let mut last: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let __decomp_4: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut first: () = unimplemented!();
        let mut last: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let __decomp_5: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut first: () = unimplemented!();
        let mut last: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let __decomp_6: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut first: () = unimplemented!();
        let mut last: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_unsized_subrange_7() {
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_()])));
    let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    {
        let __decomp_8: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut first: () = unimplemented!();
        let last: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let __decomp_9: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut first: () = unimplemented!();
        let last: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let __decomp_10: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut first: () = unimplemented!();
        let last: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let __decomp_11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let mut first: () = unimplemented!();
        let last: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        unimplemented!();
        const _: () = assert!(true, "");;
    }
}
pub fn test_copies_not_originals_12() {
    let a: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([1_(), 2_(), 3_(), 4_()])));
    {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let __decomp_13: Ptr<()> = r.as_pointer();
        let mut first: () = unimplemented!();
        let mut last: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        first = (a.as_pointer() as Ptr<()>).offset((2_()) as isize);
        last = (a.as_pointer() as Ptr<()>).offset((2_()) as isize);
        unimplemented!();
        unimplemented!();
    }
    {
        let r: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let __decomp_14: Ptr<()> = r.as_pointer();
        let mut first: () = unimplemented!();
        let mut last: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        first = (a.as_pointer() as Ptr<()>).offset((2_()) as isize);
        last = (a.as_pointer() as Ptr<()>).offset((2_()) as isize);
        unimplemented!();
        unimplemented!();
    }
}
pub fn test_15() -> () {
    ({ test_sized_subrange_2() });
    ({ test_unsized_subrange_7() });
    ({ test_copies_not_originals_12() });
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
    ({ test_15() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
