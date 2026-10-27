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
pub static mut Cases_1: std::cell::LazyCell<[anon_0; 8]> = std::cell::LazyCell::new(|| unsafe {
    [
        anon_0 {
            x: 0_(),
            y: 0_(),
            expect: 0_(),
        },
        anon_0 {
            x: 1_(),
            y: 0_(),
            expect: 0_(),
        },
        anon_0 {
            x: 0_(),
            y: 1_(),
            expect: 0_(),
        },
        anon_0 {
            x: 1_(),
            y: 1_(),
            expect: 1_(),
        },
        anon_0 {
            x: 2_(),
            y: 3_(),
            expect: 6_(),
        },
        anon_0 {
            x: 2_(),
            y: 4_(),
            expect: 4_(),
        },
        anon_0 {
            x: 3_(),
            y: 17_(),
            expect: 51_(),
        },
        anon_0 {
            x: 36_(),
            y: 18_(),
            expect: 36_(),
        },
    ]
});
pub unsafe fn test0_2(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_3(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_4(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_5(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_6(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_7(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_8(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_9(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_10(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = (in1 as ());
    let mut value2: () = (in2 as ());
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_11(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_12(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = (in1 as ());
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_13(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = (in2 as ());
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_14(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_15(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_16(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_17(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_18(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_19(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_20(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_21(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_22(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = (in2 as ());
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_23(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = (in1 as ());
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_24(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_25(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_26(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_27(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_28(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = (in1 as ());
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_29(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = (in2 as ());
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_30(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = (in2 as ());
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_31(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = (in1 as ());
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_32(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_33(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_34(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_35(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_36(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = (in1 as ());
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_37(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = (in2 as ());
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_38(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = (in1 as ());
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_39(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = (in2 as ());
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_40(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_41(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_42(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = (in1 as ());
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_43(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = (in2 as ());
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_44(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_45(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_46(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = (in1 as ());
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_47(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = (in2 as ());
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_48(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_49(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_50(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = (in1 as ());
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_51(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = (in2 as ());
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_52(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn test0_53(mut in1: (), mut in2: (), mut out: ()) -> () {
    let mut value1: () = in1;
    let mut value2: () = in2;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    unimplemented!();
    return true;
}
pub unsafe fn do_test_54(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((8_()) as isize),
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
pub unsafe fn do_test_55(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((8_()) as isize),
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
pub unsafe fn do_test_56(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((8_()) as isize),
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
pub unsafe fn do_test_57(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((8_()) as isize),
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
pub unsafe fn do_test_58(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((8_()) as isize),
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
pub unsafe fn do_test_59(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((8_()) as isize),
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
pub unsafe fn do_test_60(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((8_()) as isize),
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
pub unsafe fn do_test_61(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((8_()) as isize),
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
pub unsafe fn do_test_62(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((8_()) as isize),
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
pub unsafe fn do_test_63(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((8_()) as isize),
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
pub unsafe fn do_test_64(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((8_()) as isize),
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
pub unsafe fn do_test_65(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((8_()) as isize),
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
pub unsafe fn do_test_66(mut _a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| unsafe { 0_() });
    let mut accumulate: () = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((8_()) as isize),
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
pub unsafe fn test_limits_67() -> () {
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_limits_68() -> () {
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_limits_69() -> () {
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_limits_70() -> () {
    unimplemented!();
    unimplemented!();
    return true;
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
        let mut res1: () = unimplemented!();
        &(unimplemented!());
        const _: () = assert!(true, "std::is_same_v<decltype(res1), std::int64_t>");;
        unimplemented!();
    }
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        const _: () = assert!(true, "test_limits<unsigned int>()");;
        const _: () = assert!(true, "test_limits<std::uint32_t>()");;
        const _: () = assert!(true, "test_limits<std::uint64_t>()");;
        const _: () = assert!(true, "test_limits<int>()");;
        const _: () = assert!(true, "test_limits<std::int32_t>()");;
        const _: () = assert!(true, "test_limits<std::int64_t>()");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const Cases_1);
}
