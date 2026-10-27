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
pub struct test_macros_detail_is_same_const_char_ref__const_char_ref_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_wchar_t_ref__const_wchar_t_ref_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_char16_t_ref__const_char16_t_ref_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_const_char32_t_ref__const_char32_t_ref_ {}
pub fn test_4(mut s: Ptr<()>, mut len: ()) {
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < len) {
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test_5(mut s: Ptr<()>, mut len: ()) {
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < len) {
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test_6(mut s: Ptr<()>, mut len: ()) {
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < len) {
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
}
pub fn test_7(mut s: Ptr<()>, mut len: ()) {
    let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while (i < len) {
        unimplemented!();
        unimplemented!();
        i.prefix_inc();
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_4(Ptr::<()>::from_string_literal(b"ABCDE"), 5_()) });
    ({ test_4(Ptr::<()>::from_string_literal(b"a"), 1_()) });
    ({
        test_5(
            Ptr::<()>::from_string_literal(&[
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
            ]),
            5_(),
        )
    });
    ({ test_5(Ptr::<()>::from_string_literal(&[97 as (), 0 as ()]), 1_()) });
    ({
        test_6(
            Ptr::<()>::from_string_literal(&[
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
            ]),
            5_(),
        )
    });
    ({ test_6(Ptr::<()>::from_string_literal(&[97 as (), 0 as ()]), 1_()) });
    ({
        test_7(
            Ptr::<()>::from_string_literal(&[
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as (),
            ]),
            5_(),
        )
    });
    ({ test_7(Ptr::<()>::from_string_literal(&[97 as (), 0 as ()]), 1_()) });
    {
        let sv: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        const _: () = assert!(true, "sv.length() == 2");;
        const _: () = assert!(true, "sv[0] == \'A\'");;
        const _: () = assert!(true, "sv[1] == \'B\'");;
    }
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
