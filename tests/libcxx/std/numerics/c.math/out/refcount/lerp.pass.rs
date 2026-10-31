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
pub struct test_macros_detail_is_same_float__float_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_double__double_ {}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_long_double__long_double_ {}
pub fn constexpr_test_3() -> bool {
    return ((unimplemented!() == 0_f32) && (unimplemented!() == 6_f32))
        && (unimplemented!() == 24_f32);
}
pub fn constexpr_test_4() -> bool {
    return ((unimplemented!() == 0_f64) && (unimplemented!() == 6_f64))
        && (unimplemented!() == 24_f64);
}
pub fn constexpr_test_5() -> bool {
    return ((unimplemented!() == 0_f64) && (unimplemented!() == 6_f64))
        && (unimplemented!() == 24_f64);
}
pub fn test_6() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut maxV: f32 = f32::MAX;
    let mut inf: f32 = unimplemented!();
    assert!((unimplemented!() == 0_f32));
    assert!((unimplemented!() == 12_f32));
    assert!((unimplemented!() == 12_f32));
    assert!((unimplemented!() == 0_f32));
    assert!((unimplemented!() == 6_f32));
    assert!((unimplemented!() == 6_f32));
    assert!((unimplemented!() == 24_f32));
    assert!((unimplemented!() == (-12_i32 as f32)));
    assert!((unimplemented!() == maxV));
    assert!((unimplemented!() == maxV));
    assert!((unimplemented!() == (2.3_f64 as f32)));
    assert!((unimplemented!() == 0_f32));
    assert!(unimplemented!());
}
pub fn test_7() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut maxV: f64 = f64::MAX;
    let mut inf: f64 = unimplemented!();
    assert!((unimplemented!() == 0_f64));
    assert!((unimplemented!() == 12_f64));
    assert!((unimplemented!() == 12_f64));
    assert!((unimplemented!() == 0_f64));
    assert!((unimplemented!() == 6_f64));
    assert!((unimplemented!() == 6_f64));
    assert!((unimplemented!() == 24_f64));
    assert!((unimplemented!() == (-12_i32 as f64)));
    assert!((unimplemented!() == maxV));
    assert!((unimplemented!() == maxV));
    assert!((unimplemented!() == 2.3_f64));
    assert!((unimplemented!() == 0_f64));
    assert!(unimplemented!());
}
pub fn test_8() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut maxV: f64 = f64::MAX;
    let mut inf: f64 = unimplemented!();
    assert!((unimplemented!() == 0_f64));
    assert!((unimplemented!() == 12_f64));
    assert!((unimplemented!() == 12_f64));
    assert!((unimplemented!() == 0_f64));
    assert!((unimplemented!() == 6_f64));
    assert!((unimplemented!() == 6_f64));
    assert!((unimplemented!() == 24_f64));
    assert!((unimplemented!() == (-12_i32 as f64)));
    assert!((unimplemented!() == maxV));
    assert!((unimplemented!() == maxV));
    assert!((unimplemented!() == 2.3_f64));
    assert!((unimplemented!() == 0_f64));
    assert!(unimplemented!());
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
    const _: () = assert!(true, "constexpr_test<float>()");;
    const _: () = assert!(true, "constexpr_test<double>()");;
    const _: () = assert!(true, "constexpr_test<long double>()");;
    ({ test_6() });
    ({ test_7() });
    ({ test_8() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
