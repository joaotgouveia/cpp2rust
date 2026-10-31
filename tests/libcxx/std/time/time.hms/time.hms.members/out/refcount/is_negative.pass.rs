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
pub struct test_macros_detail_is_same_bool__bool_ {}
pub fn check_neg_1(d: ()) -> bool {
    let d: Value<()> = Rc::new(RefCell::new(d));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
}
pub fn check_neg_2(d: ()) -> bool {
    let d: Value<()> = Rc::new(RefCell::new(d));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
}
pub fn check_neg_3(d: ()) -> bool {
    let d: Value<()> = Rc::new(RefCell::new(d));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
}
pub fn check_neg_4(d: ()) -> bool {
    let d: Value<()> = Rc::new(RefCell::new(d));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
}
pub fn check_neg_5(d: ()) -> bool {
    let d: Value<()> = Rc::new(RefCell::new(d));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
}
pub fn check_neg_6(d: ()) -> bool {
    let d: Value<()> = Rc::new(RefCell::new(d));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
}
pub fn check_neg_7(d: ()) -> bool {
    let d: Value<()> = Rc::new(RefCell::new(d));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    return unimplemented!();
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
    const _: () = assert!(true, "!check_neg(std::chrono::minutes( 1))");;
    const _: () = assert!(true, "check_neg(std::chrono::minutes(-1))");;
    assert!(!({ check_neg_2(unimplemented!(),) }));
    assert!(({ check_neg_2(unimplemented!(),) }));
    assert!(!({ check_neg_1(unimplemented!(),) }));
    assert!(({ check_neg_1(unimplemented!(),) }));
    assert!(!({ check_neg_3(unimplemented!(),) }));
    assert!(({ check_neg_3(unimplemented!(),) }));
    assert!(!({ check_neg_4(unimplemented!(),) }));
    assert!(({ check_neg_4(unimplemented!(),) }));
    assert!(!({ check_neg_5(unimplemented!(),) }));
    assert!(({ check_neg_5(unimplemented!(),) }));
    assert!(!({ check_neg_6(unimplemented!(),) }));
    assert!(({ check_neg_6(unimplemented!(),) }));
    assert!(!({ check_neg_7(unimplemented!(),) }));
    assert!(({ check_neg_7(unimplemented!(),) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
