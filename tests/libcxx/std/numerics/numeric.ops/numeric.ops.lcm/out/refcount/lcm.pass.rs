extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(12)]
pub struct anon_0 {
    #[offset(0)]
    pub x: (),
    #[offset(4)]
    pub y: (),
    #[offset(8)]
    pub expect: (),
}
thread_local!(
    pub static Cases_1: Value<Box<[anon_0]>> = Rc::new(RefCell::new(Box::new([
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
    ])));
);
pub fn test0_2(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_3(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_4(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_5(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_6(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_7(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_8(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_9(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_10(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_11(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_12(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_13(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_14(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_15(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_16(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_17(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_18(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_19(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_20(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_21(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_22(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_23(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_24(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_25(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_26(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_27(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_28(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_29(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_30(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_31(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_32(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_33(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_34(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_35(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_36(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_37(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_38(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_39(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_40(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_41(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_42(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_43(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_44(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_45(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_46(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_47(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_48(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_49(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_50(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_51(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_52(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn test0_53(mut in1: (), mut in2: (), mut out: ()) -> () {
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
pub fn do_test_54(_a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| 0_());
    let accumulate: Value<()> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_2(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_2(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_2(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_2(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_2(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_2(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_2(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_2(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_3(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_3(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_4(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_5(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_4(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_5(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_4(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_5(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_4(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_5(_in1, _in2, _out)
            });
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_55(_a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| 0_());
    let accumulate: Value<()> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_6(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_6(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_6(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_6(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_6(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_6(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_6(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_6(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_7(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_7(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_8(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_9(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_8(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_9(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_8(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_9(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_8(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_9(_in1, _in2, _out)
            });
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_56(_a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| 0_());
    let accumulate: Value<()> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_10(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_10(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_10(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_10(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_10(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_10(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_10(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_10(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_11(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_11(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_12(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_13(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_12(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_13(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_12(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_13(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_12(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_13(_in1, _in2, _out)
            });
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_57(_a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| 0_());
    let accumulate: Value<()> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_14(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_14(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_14(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_14(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_14(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_14(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_14(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_14(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_15(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_15(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_16(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_17(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_16(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_17(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_16(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_17(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_16(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_17(_in1, _in2, _out)
            });
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_58(_a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| 0_());
    let accumulate: Value<()> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_18(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_18(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_18(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_18(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_18(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_18(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_18(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_18(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_19(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_19(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_20(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_21(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_20(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_21(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_20(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_21(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_20(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_21(_in1, _in2, _out)
            });
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_59(_a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| 0_());
    let accumulate: Value<()> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_22(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_22(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_22(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_22(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_23(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_23(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_23(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_23(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_24(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_25(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_26(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_27(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_26(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_27(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_28(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_29(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_28(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_29(_in1, _in2, _out)
            });
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_60(_a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| 0_());
    let accumulate: Value<()> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_23(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_23(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_23(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_23(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_22(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_22(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_22(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_22(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_25(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_24(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_28(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_29(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_28(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_29(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_26(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_27(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_26(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_27(_in1, _in2, _out)
            });
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_61(_a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| 0_());
    let accumulate: Value<()> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_30(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_30(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_30(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_30(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_31(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_31(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_31(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_31(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_32(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_33(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_34(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_35(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_34(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_35(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_36(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_37(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_36(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_37(_in1, _in2, _out)
            });
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_62(_a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| 0_());
    let accumulate: Value<()> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_31(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_31(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_31(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_31(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_30(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_30(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_30(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_30(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_33(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_32(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_36(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_37(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_36(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_37(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_34(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_35(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_34(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_35(_in1, _in2, _out)
            });
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_63(_a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| 0_());
    let accumulate: Value<()> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_38(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_38(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_38(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_38(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_39(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_39(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_39(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_39(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_40(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_41(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_42(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_43(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_42(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_43(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_44(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_45(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_44(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_45(_in1, _in2, _out)
            });
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_64(_a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| 0_());
    let accumulate: Value<()> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_39(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_39(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_39(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_39(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_38(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_38(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_38(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_38(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_41(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_40(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_44(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_45(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_44(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_45(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_42(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_43(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_42(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_43(_in1, _in2, _out)
            });
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_65(_a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| 0_());
    let accumulate: Value<()> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_46(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_46(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_46(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_46(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_47(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_47(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_47(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_47(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_48(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_49(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_50(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_51(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_50(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_51(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_52(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_53(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_52(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_53(_in1, _in2, _out)
            });
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_66(_a0: Option<()>) -> () {
    let mut _a0: () = _a0.unwrap_or_else(|| 0_());
    let accumulate: Value<()> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_()) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_47(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_47(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_47(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_47(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_46(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_46(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_46(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_46(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_49(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_48(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_52(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_53(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_52(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_53(_in1, _in2, _out)
            });
        }
        {
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_50(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_51(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = -TC.x;
                let _in2: () = TC.y;
                let _out: () = TC.expect;
                test0_50(_in1, _in2, _out)
            });
            (*accumulate.borrow_mut()) &= ({
                let _in1: () = TC.x;
                let _in2: () = -TC.y;
                let _out: () = TC.expect;
                test0_51(_in1, _in2, _out)
            });
        }
    }
    return (*accumulate.borrow());
}
pub fn test_limits_67() -> () {
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_limits_68() -> () {
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_limits_69() -> () {
    unimplemented!();
    unimplemented!();
    return true;
}
pub fn test_limits_70() -> () {
    unimplemented!();
    unimplemented!();
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
fn main_0(argc: i32, _a1: Ptr<Ptr<i8>>) -> () {
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
        let res1: Value<()> = Rc::new(RefCell::new(
            ({ lcm_71((1234_i64 as ()), (-2147483647_() - 1_())) }),
        ));
        &({ lcm_72((-2147483647_() - 1_()), 2_()) });
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
pub fn __cpp2rust_init_globals() {
    let _ = Cases_1.with(|_| ());
}
