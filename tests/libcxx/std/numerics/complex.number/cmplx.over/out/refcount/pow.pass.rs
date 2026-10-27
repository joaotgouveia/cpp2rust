extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static testcases_0: Value<Box<[()]>> = Rc::new(RefCell::new(Box::new([
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
    ])));
);
pub type anon_1 = ();
pub const anon_1_zero: anon_1 = 0;
pub const anon_1_non_zero: anon_1 = 1;
pub const anon_1_inf: anon_1 = 2;
pub const anon_1_NaN: anon_1 = 3;
pub const anon_1_non_zero_nan: anon_1 = 4;
pub fn classify_2(mut x: ()) -> () {
    if (x == 0_()) {
        return (anon_1_zero as ());
    }
    if ({ isinf_3(x) }) {
        return (anon_1_inf as ());
    }
    if ({ isnan_4(x) }) {
        return (anon_1_NaN as ());
    }
    return (anon_1_non_zero as ());
}
pub fn is_about_5(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn is_about_6(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn is_about_7(mut x: (), mut y: ()) {
    unimplemented!();
}
pub fn test_8(mut x: (), y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_9(mut x: (), y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_10(mut x: (), y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_11(mut x: (), y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_12(mut x: (), y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_13(mut x: (), y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_14(mut x: (), y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_15(mut x: (), y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_16(mut x: (), y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_17(mut x: (), y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_18(mut x: (), y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_19(mut x: (), y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_20(mut x: (), y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_21(mut x: (), y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_22(mut x: (), y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_23(x: Ptr<()>, mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_24(x: Ptr<()>, mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_25(x: Ptr<()>, mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_26(x: Ptr<()>, mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_27(x: Ptr<()>, mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_28(x: Ptr<()>, mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_29(x: Ptr<()>, mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_30(x: Ptr<()>, mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_31(x: Ptr<()>, mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_32(x: Ptr<()>, mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_33(x: Ptr<()>, mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_34(x: Ptr<()>, mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_35(x: Ptr<()>, mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_36(x: Ptr<()>, mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_37(x: Ptr<()>, mut y: ()) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_38(x: Ptr<()>, y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_39(x: Ptr<()>, y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_40(x: Ptr<()>, y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_41(x: Ptr<()>, y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_42(x: Ptr<()>, y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_43(x: Ptr<()>, y: Ptr<()>) {
    const _: () = assert!(
        true,
        "(std::is_same<decltype(std::pow(x, y)), std::complex<V> >::value)"
    );;
    unimplemented!();
}
pub fn test_44(_a0: Option<()>, _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    ({
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_8(3_(), _y.as_pointer())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_23(_x.as_pointer(), 5_())
    });
}
pub fn test_45(_a0: Option<()>, _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    ({
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_9(3_(), _y.as_pointer())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_24(_x.as_pointer(), 5_())
    });
}
pub fn test_46(_a0: Option<()>, _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    ({
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_10(3_(), _y.as_pointer())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_25(_x.as_pointer(), 5_())
    });
}
pub fn test_47(_a0: Option<()>, _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    ({
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_11(3_(), _y.as_pointer())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_26(_x.as_pointer(), 5_())
    });
}
pub fn test_48(_a0: Option<()>, _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    ({
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_12(3_(), _y.as_pointer())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_27(_x.as_pointer(), 5_())
    });
}
pub fn test_49(_a0: Option<()>, _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    ({
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_13(3_(), _y.as_pointer())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_28(_x.as_pointer(), 5_())
    });
}
pub fn test_50(_a0: Option<()>, _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    ({
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_14(3_(), _y.as_pointer())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_29(_x.as_pointer(), 5_())
    });
}
pub fn test_51(_a0: Option<()>, _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    ({
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_15(3_(), _y.as_pointer())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_30(_x.as_pointer(), 5_())
    });
}
pub fn test_52(_a0: Option<()>, _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    ({
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_16(3_(), _y.as_pointer())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_31(_x.as_pointer(), 5_())
    });
}
pub fn test_53(_a0: Option<()>, _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    ({
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_17(3_(), _y.as_pointer())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_32(_x.as_pointer(), 5_())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_38(_x.as_pointer(), _y.as_pointer())
    });
}
pub fn test_54(_a0: Option<()>, _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    ({
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_18(3_(), _y.as_pointer())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_33(_x.as_pointer(), 5_())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_39(_x.as_pointer(), _y.as_pointer())
    });
}
pub fn test_55(_a0: Option<()>, _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    ({
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_19(3_(), _y.as_pointer())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_34(_x.as_pointer(), 5_())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_40(_x.as_pointer(), _y.as_pointer())
    });
}
pub fn test_56(_a0: Option<()>, _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    ({
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_20(3_(), _y.as_pointer())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_35(_x.as_pointer(), 5_())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_41(_x.as_pointer(), _y.as_pointer())
    });
}
pub fn test_57(_a0: Option<()>, _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    ({
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_21(3_(), _y.as_pointer())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_36(_x.as_pointer(), 5_())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_42(_x.as_pointer(), _y.as_pointer())
    });
}
pub fn test_58(_a0: Option<()>, _a1: Option<()>) {
    let mut _a0: () = _a0.unwrap_or_else(|| unimplemented!());
    let mut _a1: () = _a1.unwrap_or_else(|| unimplemented!());
    ({
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_22(3_(), _y.as_pointer())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_37(_x.as_pointer(), 5_())
    });
    ({
        let _x: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let _y: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        test_43(_x.as_pointer(), _y.as_pointer())
    });
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
    ({ test_44(None, None) });
    ({ test_45(None, None) });
    ({ test_46(None, None) });
    ({ test_47(None, None) });
    ({ test_48(None, None) });
    ({ test_49(None, None) });
    ({ test_50(None, None) });
    ({ test_51(None, None) });
    ({ test_52(None, None) });
    ({ test_53(None, None) });
    ({ test_54(None, None) });
    ({ test_55(None, None) });
    ({ test_56(None, None) });
    ({ test_57(None, None) });
    ({ test_58(None, None) });
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = testcases_0.with(|_| ());
}
