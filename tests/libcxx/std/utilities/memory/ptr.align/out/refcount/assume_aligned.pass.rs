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
pub struct test_macros_detail_is_same_char_ptr__char_ptr_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_long_ptr__long_ptr_ {}
pub type anon_3 = ();
pub const anon_3_value: anon_3 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_double_ptr__double_ptr_ {}
pub type anon_4 = ();
pub const anon_4_value: anon_4 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_long_double_ptr__long_double_ptr_ {}
pub type anon_5 = ();
pub const anon_5_value: anon_5 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_S_ptr__S_ptr_ {}
pub type anon_6 = ();
pub const anon_6_value: anon_6 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_S4_ptr__S4_ptr_ {}
pub type anon_7 = ();
pub const anon_7_value: anon_7 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_S8_ptr__S8_ptr_ {}
pub type anon_8 = ();
pub const anon_8_value: anon_8 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_S16_ptr__S16_ptr_ {}
pub type anon_9 = ();
pub const anon_9_value: anon_9 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_S32_ptr__S32_ptr_ {}
pub type anon_10 = ();
pub const anon_10_value: anon_10 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_S64_ptr__S64_ptr_ {}
pub type anon_11 = ();
pub const anon_11_value: anon_11 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_S128_ptr__S128_ptr_ {}
pub fn check_12(mut p: Ptr<()>) {
    const _: () = assert!(true, "");;
    let mut alignment: () = ::std::mem::align_of::<()>();
    {
        unimplemented!();
    }
    {}
    {}
    {}
    {}
    {}
    {}
    {}
}
pub fn check_13(mut p: Ptr<()>) {
    const _: () = assert!(true, "");;
    let mut alignment: () = ::std::mem::align_of::<()>();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    {}
    {}
    {}
    {}
}
pub fn check_14(mut p: Ptr<()>) {
    const _: () = assert!(true, "");;
    let mut alignment: () = ::std::mem::align_of::<()>();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    {}
    {}
    {}
}
pub fn check_15(mut p: Ptr<()>) {
    const _: () = assert!(true, "");;
    let mut alignment: () = ::std::mem::align_of::<()>();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    {}
    {}
    {}
}
pub fn check_16(mut p: Ptr<()>) {
    const _: () = assert!(true, "");;
    let mut alignment: () = ::std::mem::align_of::<()>();
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    {}
    {}
}
pub fn check_17(mut p: Ptr<S>) {
    const _: () = assert!(true, "");;
    let mut alignment: () = 1usize;
    {
        unimplemented!();
    }
    {}
    {}
    {}
    {}
    {}
    {}
    {}
}
pub fn check_18(mut p: Ptr<S4>) {
    const _: () = assert!(true, "");;
    let mut alignment: () = 4usize;
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    {}
    {}
    {}
    {}
}
pub fn check_19(mut p: Ptr<S8>) {
    const _: () = assert!(true, "");;
    let mut alignment: () = 8usize;
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    {}
    {}
    {}
}
pub fn check_20(mut p: Ptr<S16>) {
    const _: () = assert!(true, "");;
    let mut alignment: () = 16usize;
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    {}
    {}
}
pub fn check_21(mut p: Ptr<S32>) {
    const _: () = assert!(true, "");;
    let mut alignment: () = 32usize;
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
    {}
}
pub fn check_22(mut p: Ptr<S64>) {
    const _: () = assert!(true, "");;
    let mut alignment: () = 64usize;
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {}
}
pub fn check_23(mut p: Ptr<S128>) {
    const _: () = assert!(true, "");;
    let mut alignment: () = 128usize;
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
    {
        unimplemented!();
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct S {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S4 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct S8 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct S16 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(32)]
pub struct S32 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(64)]
pub struct S64 {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(128)]
pub struct S128 {}
pub fn tests_24() -> () {
    let c: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let i: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let l: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let d: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    let ld: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    ({ check_12((c.as_pointer())) });
    ({ check_13((i.as_pointer())) });
    ({ check_14((l.as_pointer())) });
    ({ check_15((d.as_pointer())) });
    ({ check_16((ld.as_pointer())) });
    let s: Value<S> = Rc::new(RefCell::new(<S>::default()));
    let s4: Value<S4> = Rc::new(RefCell::new(<S4>::default()));
    let s8: Value<S8> = Rc::new(RefCell::new(<S8>::default()));
    let s16: Value<S16> = Rc::new(RefCell::new(<S16>::default()));
    let s32: Value<S32> = Rc::new(RefCell::new(<S32>::default()));
    let s64: Value<S64> = Rc::new(RefCell::new(<S64>::default()));
    let s128: Value<S128> = Rc::new(RefCell::new(<S128>::default()));
    ({ check_17((s.as_pointer())) });
    ({ check_18((s4.as_pointer())) });
    ({ check_19((s8.as_pointer())) });
    ({ check_20((s16.as_pointer())) });
    ({ check_21((s32.as_pointer())) });
    ({ check_22((s64.as_pointer())) });
    ({ check_23((s128.as_pointer())) });
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
    ({ tests_24() });
    const _: () = assert!(true, "tests()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
