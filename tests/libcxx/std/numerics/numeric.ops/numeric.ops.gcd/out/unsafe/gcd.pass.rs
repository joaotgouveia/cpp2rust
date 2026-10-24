extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_0 {
    pub x: (),
    pub y: (),
    pub expect: (),
}
pub static mut Cases_1: std::cell::LazyCell<[anon_0; 12]> = std::cell::LazyCell::new(|| unsafe {
    [
        anon_0 {
            x: 0_(),
            y: 0_(),
            expect: 0_(),
        },
        anon_0 {
            x: 1_(),
            y: 0_(),
            expect: 1_(),
        },
        anon_0 {
            x: 0_(),
            y: 1_(),
            expect: 1_(),
        },
        anon_0 {
            x: 1_(),
            y: 1_(),
            expect: 1_(),
        },
        anon_0 {
            x: 2_(),
            y: 3_(),
            expect: 1_(),
        },
        anon_0 {
            x: 2_(),
            y: 4_(),
            expect: 2_(),
        },
        anon_0 {
            x: 11_(),
            y: 9_(),
            expect: 1_(),
        },
        anon_0 {
            x: 36_(),
            y: 17_(),
            expect: 1_(),
        },
        anon_0 {
            x: 36_(),
            y: 18_(),
            expect: 18_(),
        },
        anon_0 {
            x: 25_(),
            y: 30_(),
            expect: 5_(),
        },
        anon_0 {
            x: 24_(),
            y: 16_(),
            expect: 8_(),
        },
        anon_0 {
            x: 124_(),
            y: 100_(),
            expect: 4_(),
        },
    ]
});
pub unsafe fn test0_2(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_3(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_4(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_5(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_6(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_7(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_8(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_9(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_10(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = (in1 as ());
    let mut value2: () = (in2 as ());
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_11(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_12(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = (in1 as ());
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_13(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = (in2 as ());
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_14(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_15(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_16(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_17(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_18(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_19(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_20(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_21(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_22(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = (in2 as ());
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_23(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = (in1 as ());
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_24(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_25(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_26(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_27(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_28(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = (in1 as ());
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_29(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = (in2 as ());
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_30(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = (in2 as ());
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_31(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = (in1 as ());
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_32(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_33(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_34(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_35(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_36(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = (in1 as ());
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_37(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = (in2 as ());
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_38(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = (in1 as ());
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_39(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = (in2 as ());
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_40(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_41(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_42(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = (in1 as ());
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_43(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = (in2 as ());
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_44(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_45(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_46(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = (in1 as ());
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_47(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = (in2 as ());
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_48(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_49(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_50(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = (in1 as ());
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_51(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = (in2 as ());
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_52(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_53(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn basic_gcd__54(mut m: (), mut n: ()) -> () {
    return if ((n) == (0_())) {
        m
    } else {
        (unsafe {
            let _m: () = n;
            let _n: () = ((m) % (n));
            basic_gcd__54(_m, _n)
        })
    };
}
pub unsafe fn basic_gcd__55(mut m: (), mut n: ()) -> () {
    return if ((n) == (0_())) {
        m
    } else {
        (unsafe {
            let _m: () = n;
            let _n: () = ((m) % (n));
            basic_gcd__55(_m, _n)
        })
    };
}
pub unsafe fn basic_gcd__56(mut m: (), mut n: ()) -> () {
    return if ((n) == (0_())) {
        m
    } else {
        (unsafe {
            let _m: () = n;
            let _n: () = (m).wrapping_rem(n);
            basic_gcd__56(_m, _n)
        })
    };
}
pub unsafe fn basic_gcd__57(mut m: (), mut n: ()) -> () {
    return if ((n) == (0_())) {
        m
    } else {
        (unsafe {
            let _m: () = n;
            let _n: () = (m).wrapping_rem(n);
            basic_gcd__57(_m, _n)
        })
    };
}
pub unsafe fn basic_gcd_58(mut m: (), mut n: ()) -> () {
    {
        if ((m) < (0_())) && ((m) != (unimplemented!())) {
            m = -m;
        }
        if ((n) < (0_())) && ((n) != (unimplemented!())) {
            n = -n;
        }
    }
    return (unsafe { basic_gcd__54(m, n) });
}
pub unsafe fn basic_gcd_59(mut m: (), mut n: ()) -> () {
    {
        if ((m) < (0_())) && ((m) != (unimplemented!())) {
            m = -m;
        }
        if ((n) < (0_())) && ((n) != (unimplemented!())) {
            n = -n;
        }
    }
    return (unsafe { basic_gcd__55(m, n) });
}
pub unsafe fn basic_gcd_60(mut m: (), mut n: ()) -> () {
    {
        if ((m) < (0_())) && ((m) != (unimplemented!())) {
            m = -m;
        }
        if ((n) < (0_())) && ((n) != (unimplemented!())) {
            n = -n;
        }
    }
    return (unsafe { basic_gcd__56(m, n) });
}
pub unsafe fn basic_gcd_61(mut m: (), mut n: ()) -> () {
    {
        if ((m) < (0_())) && ((m) != (unimplemented!())) {
            m = -m;
        }
        if ((n) < (0_())) && ((n) != (unimplemented!())) {
            n = -n;
        }
    }
    return (unsafe { basic_gcd__57(m, n) });
}
pub unsafe fn basic_gcd_62(mut m: (), mut n: ()) -> () {
    {}
    return (unsafe { basic_gcd__54((m as ()), (n as ())) });
}
pub unsafe fn basic_gcd_63(mut m: (), mut n: ()) -> () {
    {}
    return (unsafe { basic_gcd__55((m as ()), (n as ())) });
}
pub unsafe fn basic_gcd_64(mut m: (), mut n: ()) -> () {
    {}
    return (unsafe { basic_gcd__56((m as ()), (n as ())) });
}
pub unsafe fn basic_gcd_65(mut m: (), mut n: ()) -> () {
    {}
    return (unsafe { basic_gcd__57((m as ()), (n as ())) });
}
pub unsafe fn do_fuzzy_tests_66() {
    let mut gen_: () = unimplemented!();
    let max_input: () = unimplemented!();
    let mut distrib: () = unimplemented!();
    let nb_rounds: () = 10000_();
    let mut i: () = 0_();
    'loop_: while ((i) < (nb_rounds)) {
        let mut n: () = unimplemented!();
        let mut m: () = unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn do_fuzzy_tests_67() {
    let mut gen_: () = unimplemented!();
    let max_input: () = unimplemented!();
    let mut distrib: () = unimplemented!();
    let nb_rounds: () = 10000_();
    let mut i: () = 0_();
    'loop_: while ((i) < (nb_rounds)) {
        let mut n: () = (unimplemented!() as ());
        let mut m: () = (unimplemented!() as ());
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn do_fuzzy_tests_68() {
    let mut gen_: () = unimplemented!();
    let max_input: () = unimplemented!();
    let mut distrib: () = unimplemented!();
    let nb_rounds: () = 10000_();
    let mut i: () = 0_();
    'loop_: while ((i) < (nb_rounds)) {
        let mut n: () = (unimplemented!() as ());
        let mut m: () = (unimplemented!() as ());
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn do_fuzzy_tests_69() {
    let mut gen_: () = unimplemented!();
    let max_input: () = unimplemented!();
    let mut distrib: () = unimplemented!();
    let nb_rounds: () = 10000_();
    let mut i: () = 0_();
    'loop_: while ((i) < (nb_rounds)) {
        let mut n: () = (unimplemented!() as ());
        let mut m: () = (unimplemented!() as ());
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn do_fuzzy_tests_70() {
    let mut gen_: () = unimplemented!();
    let max_input: () = unimplemented!();
    let mut distrib: () = unimplemented!();
    let nb_rounds: () = 10000_();
    let mut i: () = 0_();
    'loop_: while ((i) < (nb_rounds)) {
        let mut n: () = unimplemented!();
        let mut m: () = unimplemented!();
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn do_fuzzy_tests_71() {
    let mut gen_: () = unimplemented!();
    let max_input: () = unimplemented!();
    let mut distrib: () = unimplemented!();
    let nb_rounds: () = 10000_();
    let mut i: () = 0_();
    'loop_: while ((i) < (nb_rounds)) {
        let mut n: () = (unimplemented!() as ());
        let mut m: () = (unimplemented!() as ());
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn do_fuzzy_tests_72() {
    let mut gen_: () = unimplemented!();
    let max_input: () = unimplemented!();
    let mut distrib: () = unimplemented!();
    let nb_rounds: () = 10000_();
    let mut i: () = 0_();
    'loop_: while ((i) < (nb_rounds)) {
        let mut n: () = (unimplemented!() as ());
        let mut m: () = (unimplemented!() as ());
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn do_fuzzy_tests_73() {
    let mut gen_: () = unimplemented!();
    let max_input: () = unimplemented!();
    let mut distrib: () = unimplemented!();
    let nb_rounds: () = 10000_();
    let mut i: () = 0_();
    'loop_: while ((i) < (nb_rounds)) {
        let mut n: () = (unimplemented!() as ());
        let mut m: () = (unimplemented!() as ());
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn do_limit_tests_74() {
    let mut inputs: [(); 27] = [
        ((unimplemented!())
            + (if (*std::cell::LazyCell::force_mut(&mut *&raw mut value_75)) {
                3_()
            } else {
                0_()
            })),
        ((unimplemented!()) + (1_())),
        ((unimplemented!()) + (2_())),
        unimplemented!(),
        ((unimplemented!()) - (1_())),
        ((unimplemented!()) - (2_())),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
        5_(),
        6_(),
        7_(),
        8_(),
        9_(),
        10_(),
        -1_(),
        -2_(),
        -3_(),
        -4_(),
        -5_(),
        -6_(),
        -7_(),
        -8_(),
        -9_(),
        -10_(),
    ];
    'loop_: for n in IterRange::new(
        inputs.as_mut_ptr(),
        inputs.as_mut_ptr().offset((27_()) as isize),
    ) {
        let mut n: () = (*n);
        'loop_: for m in IterRange::new(
            inputs.as_mut_ptr(),
            inputs.as_mut_ptr().offset((27_()) as isize),
        ) {
            let mut m: () = (*m);
            unimplemented!();
        }
    }
}
pub unsafe fn do_limit_tests_76() {
    let mut inputs: [(); 27] = [
        ((unimplemented!())
            + (if (*std::cell::LazyCell::force_mut(&mut *&raw mut value_75)) {
                3_()
            } else {
                0_()
            })),
        ((unimplemented!()) + (1_())),
        ((unimplemented!()) + (2_())),
        unimplemented!(),
        ((unimplemented!()) - (1_())),
        ((unimplemented!()) - (2_())),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
        5_(),
        6_(),
        7_(),
        8_(),
        9_(),
        10_(),
        -1_(),
        -2_(),
        -3_(),
        -4_(),
        -5_(),
        -6_(),
        -7_(),
        -8_(),
        -9_(),
        -10_(),
    ];
    'loop_: for n in IterRange::new(
        inputs.as_mut_ptr(),
        inputs.as_mut_ptr().offset((27_()) as isize),
    ) {
        let mut n: () = (*n);
        'loop_: for m in IterRange::new(
            inputs.as_mut_ptr(),
            inputs.as_mut_ptr().offset((27_()) as isize),
        ) {
            let mut m: () = (*m);
            unimplemented!();
        }
    }
}
pub unsafe fn do_limit_tests_77() {
    let mut inputs: [(); 27] = [
        ((unimplemented!())
            + (if (*std::cell::LazyCell::force_mut(&mut *&raw mut value_75)) {
                3_()
            } else {
                0_()
            })),
        ((unimplemented!()) + (1_())),
        ((unimplemented!()) + (2_())),
        unimplemented!(),
        ((unimplemented!()) - (1_())),
        ((unimplemented!()) - (2_())),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
        5_(),
        6_(),
        7_(),
        8_(),
        9_(),
        10_(),
        (-1_() as ()),
        (-2_() as ()),
        (-3_() as ()),
        (-4_() as ()),
        (-5_() as ()),
        (-6_() as ()),
        (-7_() as ()),
        (-8_() as ()),
        (-9_() as ()),
        (-10_() as ()),
    ];
    'loop_: for n in IterRange::new(
        inputs.as_mut_ptr(),
        inputs.as_mut_ptr().offset((27_()) as isize),
    ) {
        let mut n: () = (*n);
        'loop_: for m in IterRange::new(
            inputs.as_mut_ptr(),
            inputs.as_mut_ptr().offset((27_()) as isize),
        ) {
            let mut m: () = (*m);
            unimplemented!();
        }
    }
}
pub unsafe fn do_limit_tests_78() {
    let mut inputs: [(); 27] = [
        ((unimplemented!())
            + (if (*std::cell::LazyCell::force_mut(&mut *&raw mut value_75)) {
                3_()
            } else {
                0_()
            })),
        ((unimplemented!()) + (1_())),
        ((unimplemented!()) + (2_())),
        unimplemented!(),
        ((unimplemented!()) - (1_())),
        ((unimplemented!()) - (2_())),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
        5_(),
        6_(),
        7_(),
        8_(),
        9_(),
        10_(),
        -1_(),
        -2_(),
        -3_(),
        -4_(),
        -5_(),
        -6_(),
        -7_(),
        -8_(),
        -9_(),
        -10_(),
    ];
    'loop_: for n in IterRange::new(
        inputs.as_mut_ptr(),
        inputs.as_mut_ptr().offset((27_()) as isize),
    ) {
        let mut n: () = (*n);
        'loop_: for m in IterRange::new(
            inputs.as_mut_ptr(),
            inputs.as_mut_ptr().offset((27_()) as isize),
        ) {
            let mut m: () = (*m);
            unimplemented!();
        }
    }
}
pub unsafe fn do_limit_tests_79() {
    let mut inputs: [(); 27] = [
        ((unimplemented!())
            + (if (*std::cell::LazyCell::force_mut(&mut *&raw mut value_80)) {
                3_()
            } else {
                0_()
            })),
        ((unimplemented!()) + (1_())),
        ((unimplemented!()) + (2_())),
        unimplemented!(),
        ((unimplemented!()) - (1_())),
        ((unimplemented!()) - (2_())),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
        5_(),
        6_(),
        7_(),
        8_(),
        9_(),
        10_(),
        -1_(),
        -2_(),
        -3_(),
        -4_(),
        -5_(),
        -6_(),
        -7_(),
        -8_(),
        -9_(),
        -10_(),
    ];
    'loop_: for n in IterRange::new(
        inputs.as_mut_ptr(),
        inputs.as_mut_ptr().offset((27_()) as isize),
    ) {
        let mut n: () = (*n);
        'loop_: for m in IterRange::new(
            inputs.as_mut_ptr(),
            inputs.as_mut_ptr().offset((27_()) as isize),
        ) {
            let mut m: () = (*m);
            unimplemented!();
        }
    }
}
pub unsafe fn do_limit_tests_81() {
    let mut inputs: [(); 27] = [
        ((unimplemented!())
            + (if (*std::cell::LazyCell::force_mut(&mut *&raw mut value_80)) {
                3_()
            } else {
                0_()
            })),
        ((unimplemented!()) + (1_())),
        ((unimplemented!()) + (2_())),
        unimplemented!(),
        ((unimplemented!()) - (1_())),
        ((unimplemented!()) - (2_())),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
        5_(),
        6_(),
        7_(),
        8_(),
        9_(),
        10_(),
        -1_(),
        -2_(),
        -3_(),
        -4_(),
        -5_(),
        -6_(),
        -7_(),
        -8_(),
        -9_(),
        -10_(),
    ];
    'loop_: for n in IterRange::new(
        inputs.as_mut_ptr(),
        inputs.as_mut_ptr().offset((27_()) as isize),
    ) {
        let mut n: () = (*n);
        'loop_: for m in IterRange::new(
            inputs.as_mut_ptr(),
            inputs.as_mut_ptr().offset((27_()) as isize),
        ) {
            let mut m: () = (*m);
            unimplemented!();
        }
    }
}
pub unsafe fn do_limit_tests_82() {
    let mut inputs: [(); 27] = [
        (unimplemented!() as ()).wrapping_add(
            (if (*std::cell::LazyCell::force_mut(&mut *&raw mut value_80)) {
                3_()
            } else {
                0_()
            }),
        ),
        (unimplemented!() as ()).wrapping_add(1_()),
        (unimplemented!() as ()).wrapping_add(2_()),
        unimplemented!(),
        (unimplemented!() as ()).wrapping_sub(1_()),
        (unimplemented!() as ()).wrapping_sub(2_()),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
        5_(),
        6_(),
        7_(),
        8_(),
        9_(),
        10_(),
        -1_(),
        -2_(),
        -3_(),
        -4_(),
        -5_(),
        -6_(),
        -7_(),
        -8_(),
        -9_(),
        -10_(),
    ];
    'loop_: for n in IterRange::new(
        inputs.as_mut_ptr(),
        inputs.as_mut_ptr().offset((27_()) as isize),
    ) {
        let mut n: () = (*n);
        'loop_: for m in IterRange::new(
            inputs.as_mut_ptr(),
            inputs.as_mut_ptr().offset((27_()) as isize),
        ) {
            let mut m: () = (*m);
            unimplemented!();
        }
    }
}
pub unsafe fn do_limit_tests_83() {
    let mut inputs: [(); 27] = [
        (unimplemented!() as ()).wrapping_add(
            (if (*std::cell::LazyCell::force_mut(&mut *&raw mut value_80)) {
                3_()
            } else {
                0_()
            }),
        ),
        (unimplemented!() as ()).wrapping_add(1_()),
        (unimplemented!() as ()).wrapping_add(2_()),
        unimplemented!(),
        (unimplemented!() as ()).wrapping_sub(1_()),
        (unimplemented!() as ()).wrapping_sub(2_()),
        0_(),
        1_(),
        2_(),
        3_(),
        4_(),
        5_(),
        6_(),
        7_(),
        8_(),
        9_(),
        10_(),
        -1_(),
        -2_(),
        -3_(),
        -4_(),
        -5_(),
        -6_(),
        -7_(),
        -8_(),
        -9_(),
        -10_(),
    ];
    'loop_: for n in IterRange::new(
        inputs.as_mut_ptr(),
        inputs.as_mut_ptr().offset((27_()) as isize),
    ) {
        let mut n: () = (*n);
        'loop_: for m in IterRange::new(
            inputs.as_mut_ptr(),
            inputs.as_mut_ptr().offset((27_()) as isize),
        ) {
            let mut m: () = (*m);
            unimplemented!();
        }
    }
}
pub unsafe fn do_test_84(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_2(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_2(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_2(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_2(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_2(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_2(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_2(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_2(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_3(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_3(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_4(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_5(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_4(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_5(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_4(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_5(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_4(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_5(_in1, _in2, _out)
            });
        }
    }
    return accumulate;
}
pub unsafe fn do_test_85(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_6(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_6(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_6(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_6(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_6(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_6(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_6(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_6(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_7(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_7(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_8(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_9(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_8(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_9(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_8(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_9(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_8(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_9(_in1, _in2, _out)
            });
        }
    }
    return accumulate;
}
pub unsafe fn do_test_86(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_10(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_10(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_10(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_10(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_10(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_10(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_10(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_10(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_11(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_11(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_12(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_13(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_12(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_13(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_12(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_13(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_12(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_13(_in1, _in2, _out)
            });
        }
    }
    return accumulate;
}
pub unsafe fn do_test_87(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_14(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_14(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_14(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_14(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_14(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_14(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_14(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_14(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_15(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_15(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_16(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_17(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_16(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_17(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_16(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_17(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_16(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_17(_in1, _in2, _out)
            });
        }
    }
    return accumulate;
}
pub unsafe fn do_test_88(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_18(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_18(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_18(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_18(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_18(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_18(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_18(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_18(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_19(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_19(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_20(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_21(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_20(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_21(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_20(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_21(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_20(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_21(_in1, _in2, _out)
            });
        }
    }
    return accumulate;
}
pub unsafe fn do_test_89(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_22(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_22(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_22(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_22(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_23(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_23(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_23(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_23(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_24(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_25(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_26(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_27(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_26(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_27(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_28(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_29(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_28(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_29(_in1, _in2, _out)
            });
        }
    }
    return accumulate;
}
pub unsafe fn do_test_90(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_23(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_23(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_23(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_23(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_22(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_22(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_22(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_22(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_25(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_24(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_28(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_29(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_28(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_29(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_26(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_27(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_26(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_27(_in1, _in2, _out)
            });
        }
    }
    return accumulate;
}
pub unsafe fn do_test_91(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_30(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_30(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_30(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_30(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_31(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_31(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_31(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_31(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_32(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_33(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_34(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_35(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_34(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_35(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_36(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_37(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_36(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_37(_in1, _in2, _out)
            });
        }
    }
    return accumulate;
}
pub unsafe fn do_test_92(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_31(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_31(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_31(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_31(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_30(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_30(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_30(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_30(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_33(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_32(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_36(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_37(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_36(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_37(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_34(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_35(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_34(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_35(_in1, _in2, _out)
            });
        }
    }
    return accumulate;
}
pub unsafe fn do_test_93(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_38(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_38(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_38(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_38(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_39(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_39(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_39(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_39(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_40(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_41(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_42(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_43(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_42(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_43(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_44(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_45(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_44(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_45(_in1, _in2, _out)
            });
        }
    }
    return accumulate;
}
pub unsafe fn do_test_94(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_39(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_39(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_39(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_39(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_38(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_38(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_38(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_38(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_41(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_40(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_44(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_45(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_44(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_45(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_42(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_43(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_42(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_43(_in1, _in2, _out)
            });
        }
    }
    return accumulate;
}
pub unsafe fn do_test_95(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_46(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_46(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_46(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_46(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_47(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_47(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_47(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_47(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_48(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_49(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_50(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_51(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_50(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_51(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_52(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_53(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_52(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_53(_in1, _in2, _out)
            });
        }
    }
    return accumulate;
}
pub unsafe fn do_test_96(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_47(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_47(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_47(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_47(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_46(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_46(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_46(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_46(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_49(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_48(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_52(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_53(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_52(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_53(_in1, _in2, _out)
            });
        }
        {
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_50(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_51(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_50(_in1, _in2, _out)
            });
            accumulate &= (unsafe {
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_51(_in1, _in2, _out)
            });
        }
    }
    return accumulate;
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
unsafe fn main_0(mut argc: (), mut _a1: *mut *mut ()) -> () {
    let mut non_cce: () = argc;
    const _: () = assert!(true, "do_test<signed char>()");;
    const _: () = assert!(true, "do_test<short>()");;
    const _: () = assert!(true, "do_test<int>()");;
    const _: () = assert!(true, "do_test<long>()");;
    const _: () = assert!(true, "do_test<long long>()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "do_test<std::int8_t>()");;
    const _: () = assert!(true, "do_test<std::int16_t>()");;
    const _: () = assert!(true, "do_test<std::int32_t>()");;
    const _: () = assert!(true, "do_test<std::int64_t>()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "do_test<signed char, int>()");;
    const _: () = assert!(true, "do_test<int, signed char>()");;
    const _: () = assert!(true, "do_test<short, int>()");;
    const _: () = assert!(true, "do_test<int, short>()");;
    const _: () = assert!(true, "do_test<int, long>()");;
    const _: () = assert!(true, "do_test<long, int>()");;
    const _: () = assert!(true, "do_test<int, long long>()");;
    const _: () = assert!(true, "do_test<long long, int>()");;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        let mut res: () = unimplemented!();
        const _: () = assert!(true, "std::is_same_v<decltype(res), std::int64_t>");;
        unimplemented!();
    }
    (unsafe { do_fuzzy_tests_66() });
    (unsafe { do_fuzzy_tests_67() });
    (unsafe { do_fuzzy_tests_68() });
    (unsafe { do_fuzzy_tests_69() });
    (unsafe { do_fuzzy_tests_70() });
    (unsafe { do_fuzzy_tests_71() });
    (unsafe { do_fuzzy_tests_72() });
    (unsafe { do_fuzzy_tests_73() });
    (unsafe { do_limit_tests_74() });
    (unsafe { do_limit_tests_76() });
    (unsafe { do_limit_tests_77() });
    (unsafe { do_limit_tests_78() });
    (unsafe { do_limit_tests_79() });
    (unsafe { do_limit_tests_81() });
    (unsafe { do_limit_tests_82() });
    (unsafe { do_limit_tests_83() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const Cases_1);
}
