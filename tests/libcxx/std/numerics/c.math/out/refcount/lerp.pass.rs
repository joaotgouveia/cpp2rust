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
impl CppMove for test_macros_detail_is_same_float__float_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_double__double_ {}
impl CppMove for test_macros_detail_is_same_double__double_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_long_double__long_double_ {}
impl CppMove for test_macros_detail_is_same_long_double__long_double_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn constexpr_test_3() -> bool {
    return ((({ lerp_4(0_f32, 12_f32, 0_f32) }) == 0_f32)
        && (({ lerp_4(12_f32, 0_f32, (0.5_f64 as f32)) }) == 6_f32))
        && (({ lerp_4(0_f32, 12_f32, 2_f32) }) == 24_f32);
}
pub fn constexpr_test_5() -> bool {
    return ((({ lerp_6(0_f64, 12_f64, 0_f64) }) == 0_f64)
        && (({ lerp_6(12_f64, 0_f64, 0.5_f64) }) == 6_f64))
        && (({ lerp_6(0_f64, 12_f64, 2_f64) }) == 24_f64);
}
pub fn constexpr_test_7() -> bool {
    return ((({ lerp_8(0_f64, 12_f64, 0_f64) }) == 0_f64)
        && (({ lerp_8(12_f64, 0_f64, 0.5_f64) }) == 6_f64))
        && (({ lerp_8(0_f64, 12_f64, 2_f64) }) == 24_f64);
}
pub fn test_9() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut maxV: f32 = 3.4028235e+38_f32;
    let mut inf: f32 = f32::INFINITY;
    assert!((({ lerp_4(0_f32, 12_f32, 0_f32,) }) == 0_f32));
    assert!((({ lerp_4(0_f32, 12_f32, 1_f32,) }) == 12_f32));
    assert!((({ lerp_4(12_f32, 0_f32, 0_f32,) }) == 12_f32));
    assert!((({ lerp_4(12_f32, 0_f32, 1_f32,) }) == 0_f32));
    assert!((({ lerp_4(0_f32, 12_f32, (0.5_f64 as f32),) }) == 6_f32));
    assert!((({ lerp_4(12_f32, 0_f32, (0.5_f64 as f32),) }) == 6_f32));
    assert!((({ lerp_4(0_f32, 12_f32, 2_f32,) }) == 24_f32));
    assert!((({ lerp_4(12_f32, 0_f32, 2_f32,) }) == (-12_i32 as f32)));
    assert!(
        (({
            let ___a: f32 = maxV;
            let ___b: f32 = (maxV / 10_f32);
            lerp_4(___a, ___b, 0_f32)
        }) == maxV)
    );
    assert!(
        (({
            let ___a: f32 = (maxV / 10_f32);
            let ___b: f32 = maxV;
            lerp_4(___a, ___b, 1_f32)
        }) == maxV)
    );
    assert!((({ lerp_4((2.3_f64 as f32), (2.3_f64 as f32), inf,) }) == (2.3_f64 as f32)));
    assert!((({ lerp_4(0_f32, 0_f32, 23_f32,) }) == 0_f32));
    assert!(({ isnan_10(({ lerp_4(0_f32, 0_f32, inf,) }),) }));
}
pub fn test_11() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut maxV: f64 = 1.7976931348623157e+308_f64;
    let mut inf: f64 = f64::INFINITY;
    assert!((({ lerp_6(0_f64, 12_f64, 0_f64,) }) == 0_f64));
    assert!((({ lerp_6(0_f64, 12_f64, 1_f64,) }) == 12_f64));
    assert!((({ lerp_6(12_f64, 0_f64, 0_f64,) }) == 12_f64));
    assert!((({ lerp_6(12_f64, 0_f64, 1_f64,) }) == 0_f64));
    assert!((({ lerp_6(0_f64, 12_f64, 0.5_f64,) }) == 6_f64));
    assert!((({ lerp_6(12_f64, 0_f64, 0.5_f64,) }) == 6_f64));
    assert!((({ lerp_6(0_f64, 12_f64, 2_f64,) }) == 24_f64));
    assert!((({ lerp_6(12_f64, 0_f64, 2_f64,) }) == (-12_i32 as f64)));
    assert!(
        (({
            let ___a: f64 = maxV;
            let ___b: f64 = (maxV / 10_f64);
            lerp_6(___a, ___b, 0_f64)
        }) == maxV)
    );
    assert!(
        (({
            let ___a: f64 = (maxV / 10_f64);
            let ___b: f64 = maxV;
            lerp_6(___a, ___b, 1_f64)
        }) == maxV)
    );
    assert!((({ lerp_6(2.3_f64, 2.3_f64, inf,) }) == 2.3_f64));
    assert!((({ lerp_6(0_f64, 0_f64, 23_f64,) }) == 0_f64));
    assert!(({ isnan_12(({ lerp_6(0_f64, 0_f64, inf,) }),) }));
}
pub fn test_13() {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut maxV: f64 = f64::INFINITY;
    let mut inf: f64 = f64::INFINITY;
    assert!((({ lerp_8(0_f64, 12_f64, 0_f64,) }) == 0_f64));
    assert!((({ lerp_8(0_f64, 12_f64, 1_f64,) }) == 12_f64));
    assert!((({ lerp_8(12_f64, 0_f64, 0_f64,) }) == 12_f64));
    assert!((({ lerp_8(12_f64, 0_f64, 1_f64,) }) == 0_f64));
    assert!((({ lerp_8(0_f64, 12_f64, 0.5_f64,) }) == 6_f64));
    assert!((({ lerp_8(12_f64, 0_f64, 0.5_f64,) }) == 6_f64));
    assert!((({ lerp_8(0_f64, 12_f64, 2_f64,) }) == 24_f64));
    assert!((({ lerp_8(12_f64, 0_f64, 2_f64,) }) == (-12_i32 as f64)));
    assert!(
        (({
            let ___a: f64 = maxV;
            let ___b: f64 = (maxV / 10_f64);
            lerp_8(___a, ___b, 0_f64)
        }) == maxV)
    );
    assert!(
        (({
            let ___a: f64 = (maxV / 10_f64);
            let ___b: f64 = maxV;
            lerp_8(___a, ___b, 1_f64)
        }) == maxV)
    );
    assert!((({ lerp_8(2.3_f64, 2.3_f64, inf,) }) == 2.3_f64));
    assert!((({ lerp_8(0_f64, 0_f64, 23_f64,) }) == 0_f64));
    assert!(({ isnan_14(({ lerp_8(0_f64, 0_f64, inf,) }),) }));
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
    ({ test_9() });
    ({ test_11() });
    ({ test_13() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
