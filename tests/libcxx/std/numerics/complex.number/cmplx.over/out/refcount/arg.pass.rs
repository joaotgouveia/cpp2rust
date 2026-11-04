extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static testcases_0: Value<Box<[std_complex_double_]>> = Rc::new(RefCell::new(Box::new([
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
    ])));
);
pub type anon_1 = u32;
pub const anon_1_zero: anon_1 = 0;
pub const anon_1_non_zero: anon_1 = 1;
pub const anon_1_inf: anon_1 = 2;
pub const anon_1_NaN: anon_1 = 3;
pub const anon_1_non_zero_nan: anon_1 = 4;
pub fn classify_2(mut x: f64) -> i32 {
    if (x == 0_f64) {
        return (anon_1_zero as i32);
    }
    if ({ isinf_3(x) }) {
        return (anon_1_inf as i32);
    }
    if ({ isnan_4(x) }) {
        return (anon_1_NaN as i32);
    }
    return (anon_1_non_zero as i32);
}
pub fn is_about_5(mut x: f32, mut y: f32) {
    assert!(((({ libcc2rs::abs_refcount(((x - y) / (x + y)),) }) as f64) < 1e-06_f64));
}
pub fn is_about_6(mut x: f64, mut y: f64) {
    assert!((({ libcc2rs::abs_refcount(((x - y) / (x + y)),) }) < 1e-14_f64));
}
pub fn is_about_7(mut x: f64, mut y: f64) {
    assert!((({ libcc2rs::abs_refcount(((x - y) / (x + y)),) }) < 1e-14_f64));
}
pub fn test_8(mut x: i32, _a1: Option<AnyPtr>) {
    let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(true, "(std::is_same<decltype(std::arg(x)), double>::value)");;
    assert!(
        (({ arg_9(x,) })
            == ({
                let ___z: Value<std_complex_double_> =
                    Rc::new(RefCell::new(<std_complex_double_>::default()));
                arg_10(___z.as_pointer())
            }))
    );
}
pub fn test_11(mut x: u32, _a1: Option<AnyPtr>) {
    let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(true, "(std::is_same<decltype(std::arg(x)), double>::value)");;
    assert!(
        (({ arg_12(x,) })
            == ({
                let ___z: Value<std_complex_double_> =
                    Rc::new(RefCell::new(<std_complex_double_>::default()));
                arg_10(___z.as_pointer())
            }))
    );
}
pub fn test_13(mut x: i64, _a1: Option<AnyPtr>) {
    let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(true, "(std::is_same<decltype(std::arg(x)), double>::value)");;
    assert!(
        (({ arg_14(x,) })
            == ({
                let ___z: Value<std_complex_double_> =
                    Rc::new(RefCell::new(<std_complex_double_>::default()));
                arg_10(___z.as_pointer())
            }))
    );
}
pub fn test_15(mut x: f32, _a1: Option<AnyPtr>) {
    let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(true, "(std::is_same<decltype(std::arg(x)), T>::value)");;
    assert!(
        (({ arg_16(x,) })
            == ({
                let ___z: Value<std_complex_float_> =
                    Rc::new(RefCell::new(<std_complex_float_>::default()));
                arg_17(___z.as_pointer())
            }))
    );
}
pub fn test_18(mut x: f64, _a1: Option<AnyPtr>) {
    let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(true, "(std::is_same<decltype(std::arg(x)), T>::value)");;
    assert!(
        (({ arg_19(x,) })
            == ({
                let ___z: Value<std_complex_double_> =
                    Rc::new(RefCell::new(<std_complex_double_>::default()));
                arg_10(___z.as_pointer())
            }))
    );
}
pub fn test_20(mut x: f64, _a1: Option<AnyPtr>) {
    let mut _a1: AnyPtr = _a1.unwrap_or_else(|| AnyPtr::default());
    const _: () = assert!(true, "(std::is_same<decltype(std::arg(x)), T>::value)");;
    assert!(
        (({ arg_21(x,) })
            == ({
                let ___z: Value<std_complex_long_double_> =
                    Rc::new(RefCell::new(<std_complex_long_double_>::default()));
                arg_22(___z.as_pointer())
            }))
    );
}
pub fn test_23() {
    ({ test_15(0_f32, None) });
    ({ test_15(1_f32, None) });
    ({ test_15(10_f32, None) });
}
pub fn test_24() {
    ({ test_18(0_f64, None) });
    ({ test_18(1_f64, None) });
    ({ test_18(10_f64, None) });
}
pub fn test_25() {
    ({ test_20(0_f64, None) });
    ({ test_20(1_f64, None) });
    ({ test_20(10_f64, None) });
}
pub fn test_26() {
    ({ test_8(0, None) });
    ({ test_8(1, None) });
    ({ test_8(10, None) });
}
pub fn test_27() {
    ({ test_11(0_u32, None) });
    ({ test_11(1_u32, None) });
    ({ test_11(10_u32, None) });
}
pub fn test_28() {
    ({ test_13(0_i64, None) });
    ({ test_13(1_i64, None) });
    ({ test_13(10_i64, None) });
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
    ({ test_23() });
    ({ test_24() });
    ({ test_25() });
    ({ test_26() });
    ({ test_27() });
    ({ test_28() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = testcases_0.with(|_| ());
}
