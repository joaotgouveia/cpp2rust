extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut testcases_0: std::cell::LazyCell<[(); 153]> = std::cell::LazyCell::new(|| unsafe {
    [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ]
});
pub type anon_1 = ();
pub const anon_1_zero: anon_1 = 0;
pub const anon_1_non_zero: anon_1 = 1;
pub const anon_1_inf: anon_1 = 2;
pub const anon_1_NaN: anon_1 = 3;
pub const anon_1_non_zero_nan: anon_1 = 4;
pub unsafe fn classify_2(mut x: ()) -> () {
    if ((x) == (0_())) {
        return (anon_1_zero as ());
    }
    if (unsafe { isinf_3(x) }) {
        return (anon_1_inf as ());
    }
    if (unsafe { isnan_4(x) }) {
        return (anon_1_NaN as ());
    }
    return (anon_1_non_zero as ());
}
pub unsafe fn is_about_5(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn is_about_6(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn is_about_7(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn test_8(mut x: (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_9(mut x: (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_10(mut x: (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_11(mut x: (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_12(mut x: (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_13(mut x: (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_14(mut x: (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_15(mut x: (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_16(mut x: (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_17(mut x: (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_18(mut x: (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_19(mut x: (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_20(mut x: (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_21(mut x: (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_22(mut x: (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_23(x: *const (), mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_24(x: *const (), mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_25(x: *const (), mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_26(x: *const (), mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_27(x: *const (), mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_28(x: *const (), mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_29(x: *const (), mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_30(x: *const (), mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_31(x: *const (), mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_32(x: *const (), mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_33(x: *const (), mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_34(x: *const (), mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_35(x: *const (), mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_36(x: *const (), mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_37(x: *const (), mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_38(x: *const (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_39(x: *const (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_40(x: *const (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_41(x: *const (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_42(x: *const (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_43(x: *const (), y: *const ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub unsafe fn test_44(mut _a0: Option<()>, mut _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    (unsafe {
        let mut _y: () = unimplemented!();
        test_8(3_(), &mut _y)
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        test_23(&mut _x, 5_())
    });
}
pub unsafe fn test_45(mut _a0: Option<()>, mut _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    (unsafe {
        let mut _y: () = unimplemented!();
        test_9(3_(), &mut _y)
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        test_24(&mut _x, 5_())
    });
}
pub unsafe fn test_46(mut _a0: Option<()>, mut _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    (unsafe {
        let mut _y: () = unimplemented!();
        test_10(3_(), &mut _y)
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        test_25(&mut _x, 5_())
    });
}
pub unsafe fn test_47(mut _a0: Option<()>, mut _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    (unsafe {
        let mut _y: () = unimplemented!();
        test_11(3_(), &mut _y)
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        test_26(&mut _x, 5_())
    });
}
pub unsafe fn test_48(mut _a0: Option<()>, mut _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    (unsafe {
        let mut _y: () = unimplemented!();
        test_12(3_(), &mut _y)
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        test_27(&mut _x, 5_())
    });
}
pub unsafe fn test_49(mut _a0: Option<()>, mut _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    (unsafe {
        let mut _y: () = unimplemented!();
        test_13(3_(), &mut _y)
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        test_28(&mut _x, 5_())
    });
}
pub unsafe fn test_50(mut _a0: Option<()>, mut _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    (unsafe {
        let mut _y: () = unimplemented!();
        test_14(3_(), &mut _y)
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        test_29(&mut _x, 5_())
    });
}
pub unsafe fn test_51(mut _a0: Option<()>, mut _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    (unsafe {
        let mut _y: () = unimplemented!();
        test_15(3_(), &mut _y)
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        test_30(&mut _x, 5_())
    });
}
pub unsafe fn test_52(mut _a0: Option<()>, mut _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    (unsafe {
        let mut _y: () = unimplemented!();
        test_16(3_(), &mut _y)
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        test_31(&mut _x, 5_())
    });
}
pub unsafe fn test_53(mut _a0: Option<()>, mut _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    (unsafe {
        let mut _y: () = unimplemented!();
        test_17(3_(), &mut _y)
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        test_32(&mut _x, 5_())
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        let mut _y: () = unimplemented!();
        test_38(&mut _x, &mut _y)
    });
}
pub unsafe fn test_54(mut _a0: Option<()>, mut _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    (unsafe {
        let mut _y: () = unimplemented!();
        test_18(3_(), &mut _y)
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        test_33(&mut _x, 5_())
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        let mut _y: () = unimplemented!();
        test_39(&mut _x, &mut _y)
    });
}
pub unsafe fn test_55(mut _a0: Option<()>, mut _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    (unsafe {
        let mut _y: () = unimplemented!();
        test_19(3_(), &mut _y)
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        test_34(&mut _x, 5_())
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        let mut _y: () = unimplemented!();
        test_40(&mut _x, &mut _y)
    });
}
pub unsafe fn test_56(mut _a0: Option<()>, mut _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    (unsafe {
        let mut _y: () = unimplemented!();
        test_20(3_(), &mut _y)
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        test_35(&mut _x, 5_())
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        let mut _y: () = unimplemented!();
        test_41(&mut _x, &mut _y)
    });
}
pub unsafe fn test_57(mut _a0: Option<()>, mut _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    (unsafe {
        let mut _y: () = unimplemented!();
        test_21(3_(), &mut _y)
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        test_36(&mut _x, 5_())
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        let mut _y: () = unimplemented!();
        test_42(&mut _x, &mut _y)
    });
}
pub unsafe fn test_58(mut _a0: Option<()>, mut _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
    (unsafe {
        let mut _y: () = unimplemented!();
        test_22(3_(), &mut _y)
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        test_37(&mut _x, 5_())
    });
    (unsafe {
        let mut _x: () = unimplemented!();
        let mut _y: () = unimplemented!();
        test_43(&mut _x, &mut _y)
    });
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_44(None, None) });
    (unsafe { test_45(None, None) });
    (unsafe { test_46(None, None) });
    (unsafe { test_47(None, None) });
    (unsafe { test_48(None, None) });
    (unsafe { test_49(None, None) });
    (unsafe { test_50(None, None) });
    (unsafe { test_51(None, None) });
    (unsafe { test_52(None, None) });
    (unsafe { test_53(None, None) });
    (unsafe { test_54(None, None) });
    (unsafe { test_55(None, None) });
    (unsafe { test_56(None, None) });
    (unsafe { test_57(None, None) });
    (unsafe { test_58(None, None) });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const testcases_0);
}
