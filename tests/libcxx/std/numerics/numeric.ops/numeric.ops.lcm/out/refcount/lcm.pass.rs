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
    pub x: i32,
    #[offset(4)]
    pub y: i32,
    #[offset(8)]
    pub expect: i32,
}
thread_local!(
    pub static Cases_1: Value<Box<[anon_0]>> = Rc::new(RefCell::new(Box::new([
        anon_0 {
            x: 0,
            y: 0,
            expect: 0,
        },
        anon_0 {
            x: 1,
            y: 0,
            expect: 0,
        },
        anon_0 {
            x: 0,
            y: 1,
            expect: 0,
        },
        anon_0 {
            x: 1,
            y: 1,
            expect: 1,
        },
        anon_0 {
            x: 2,
            y: 3,
            expect: 6,
        },
        anon_0 {
            x: 2,
            y: 4,
            expect: 4,
        },
        anon_0 {
            x: 3,
            y: 17,
            expect: 51,
        },
        anon_0 {
            x: 36,
            y: 18,
            expect: 36,
        },
    ])));
);
pub fn test0_2(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i8 = (in1 as i8);
    let mut value2: i8 = (in2 as i8);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!((((out as i8) as i32) == (unimplemented!() as i32)));
    return true;
}
pub fn test0_3(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u8 = (in1 as u8);
    let mut value2: u8 = (in2 as u8);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!((((out as u8) as i32) == (unimplemented!() as i32)));
    return true;
}
pub fn test0_4(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i8 = (in1 as i8);
    let mut value2: u8 = (in2 as u8);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i32) == unimplemented!()));
    return true;
}
pub fn test0_5(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u8 = (in1 as u8);
    let mut value2: i8 = (in2 as i8);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i32) == unimplemented!()));
    return true;
}
pub fn test0_6(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i16 = (in1 as i16);
    let mut value2: i16 = (in2 as i16);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!((((out as i16) as i32) == (unimplemented!() as i32)));
    return true;
}
pub fn test0_7(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u16 = (in1 as u16);
    let mut value2: u16 = (in2 as u16);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!((((out as u16) as i32) == (unimplemented!() as i32)));
    return true;
}
pub fn test0_8(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i16 = (in1 as i16);
    let mut value2: u16 = (in2 as u16);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i32) == unimplemented!()));
    return true;
}
pub fn test0_9(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u16 = (in1 as u16);
    let mut value2: i16 = (in2 as i16);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i32) == unimplemented!()));
    return true;
}
pub fn test0_10(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i32 = (in1 as i32);
    let mut value2: i32 = (in2 as i32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i32) == unimplemented!()));
    return true;
}
pub fn test0_11(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u32 = (in1 as u32);
    let mut value2: u32 = (in2 as u32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u32) == unimplemented!()));
    return true;
}
pub fn test0_12(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i32 = (in1 as i32);
    let mut value2: u32 = (in2 as u32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u32) == unimplemented!()));
    return true;
}
pub fn test0_13(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u32 = (in1 as u32);
    let mut value2: i32 = (in2 as i32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u32) == unimplemented!()));
    return true;
}
pub fn test0_14(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i64 = (in1 as i64);
    let mut value2: i64 = (in2 as i64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i64) == unimplemented!()));
    return true;
}
pub fn test0_15(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u64 = (in1 as u64);
    let mut value2: u64 = (in2 as u64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u64) == unimplemented!()));
    return true;
}
pub fn test0_16(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i64 = (in1 as i64);
    let mut value2: u64 = (in2 as u64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u64) == unimplemented!()));
    return true;
}
pub fn test0_17(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u64 = (in1 as u64);
    let mut value2: i64 = (in2 as i64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u64) == unimplemented!()));
    return true;
}
pub fn test0_18(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i64 = (in1 as i64);
    let mut value2: i64 = (in2 as i64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i64) == unimplemented!()));
    return true;
}
pub fn test0_19(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u64 = (in1 as u64);
    let mut value2: u64 = (in2 as u64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u64) == unimplemented!()));
    return true;
}
pub fn test0_20(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i64 = (in1 as i64);
    let mut value2: u64 = (in2 as u64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u64) == unimplemented!()));
    return true;
}
pub fn test0_21(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u64 = (in1 as u64);
    let mut value2: i64 = (in2 as i64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u64) == unimplemented!()));
    return true;
}
pub fn test0_22(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i8 = (in1 as i8);
    let mut value2: i32 = (in2 as i32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i32) == unimplemented!()));
    return true;
}
pub fn test0_23(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i32 = (in1 as i32);
    let mut value2: i8 = (in2 as i8);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i32) == unimplemented!()));
    return true;
}
pub fn test0_24(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u8 = (in1 as u8);
    let mut value2: u32 = (in2 as u32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u32) == unimplemented!()));
    return true;
}
pub fn test0_25(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u32 = (in1 as u32);
    let mut value2: u8 = (in2 as u8);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u32) == unimplemented!()));
    return true;
}
pub fn test0_26(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i8 = (in1 as i8);
    let mut value2: u32 = (in2 as u32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u32) == unimplemented!()));
    return true;
}
pub fn test0_27(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u32 = (in1 as u32);
    let mut value2: i8 = (in2 as i8);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u32) == unimplemented!()));
    return true;
}
pub fn test0_28(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i32 = (in1 as i32);
    let mut value2: u8 = (in2 as u8);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i32) == unimplemented!()));
    return true;
}
pub fn test0_29(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u8 = (in1 as u8);
    let mut value2: i32 = (in2 as i32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i32) == unimplemented!()));
    return true;
}
pub fn test0_30(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i16 = (in1 as i16);
    let mut value2: i32 = (in2 as i32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i32) == unimplemented!()));
    return true;
}
pub fn test0_31(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i32 = (in1 as i32);
    let mut value2: i16 = (in2 as i16);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i32) == unimplemented!()));
    return true;
}
pub fn test0_32(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u16 = (in1 as u16);
    let mut value2: u32 = (in2 as u32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u32) == unimplemented!()));
    return true;
}
pub fn test0_33(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u32 = (in1 as u32);
    let mut value2: u16 = (in2 as u16);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u32) == unimplemented!()));
    return true;
}
pub fn test0_34(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i16 = (in1 as i16);
    let mut value2: u32 = (in2 as u32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u32) == unimplemented!()));
    return true;
}
pub fn test0_35(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u32 = (in1 as u32);
    let mut value2: i16 = (in2 as i16);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u32) == unimplemented!()));
    return true;
}
pub fn test0_36(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i32 = (in1 as i32);
    let mut value2: u16 = (in2 as u16);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i32) == unimplemented!()));
    return true;
}
pub fn test0_37(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u16 = (in1 as u16);
    let mut value2: i32 = (in2 as i32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i32) == unimplemented!()));
    return true;
}
pub fn test0_38(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i32 = (in1 as i32);
    let mut value2: i64 = (in2 as i64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i64) == unimplemented!()));
    return true;
}
pub fn test0_39(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i64 = (in1 as i64);
    let mut value2: i32 = (in2 as i32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i64) == unimplemented!()));
    return true;
}
pub fn test0_40(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u32 = (in1 as u32);
    let mut value2: u64 = (in2 as u64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u64) == unimplemented!()));
    return true;
}
pub fn test0_41(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u64 = (in1 as u64);
    let mut value2: u32 = (in2 as u32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u64) == unimplemented!()));
    return true;
}
pub fn test0_42(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i32 = (in1 as i32);
    let mut value2: u64 = (in2 as u64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u64) == unimplemented!()));
    return true;
}
pub fn test0_43(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u64 = (in1 as u64);
    let mut value2: i32 = (in2 as i32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u64) == unimplemented!()));
    return true;
}
pub fn test0_44(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i64 = (in1 as i64);
    let mut value2: u32 = (in2 as u32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i64) == unimplemented!()));
    return true;
}
pub fn test0_45(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u32 = (in1 as u32);
    let mut value2: i64 = (in2 as i64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i64) == unimplemented!()));
    return true;
}
pub fn test0_46(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i32 = (in1 as i32);
    let mut value2: i64 = (in2 as i64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i64) == unimplemented!()));
    return true;
}
pub fn test0_47(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i64 = (in1 as i64);
    let mut value2: i32 = (in2 as i32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i64) == unimplemented!()));
    return true;
}
pub fn test0_48(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u32 = (in1 as u32);
    let mut value2: u64 = (in2 as u64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u64) == unimplemented!()));
    return true;
}
pub fn test0_49(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u64 = (in1 as u64);
    let mut value2: u32 = (in2 as u32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u64) == unimplemented!()));
    return true;
}
pub fn test0_50(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i32 = (in1 as i32);
    let mut value2: u64 = (in2 as u64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u64) == unimplemented!()));
    return true;
}
pub fn test0_51(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u64 = (in1 as u64);
    let mut value2: i32 = (in2 as i32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as u64) == unimplemented!()));
    return true;
}
pub fn test0_52(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i64 = (in1 as i64);
    let mut value2: u32 = (in2 as u32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i64) == unimplemented!()));
    return true;
}
pub fn test0_53(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u32 = (in1 as u32);
    let mut value2: i64 = (in2 as i64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::lcm(value2, value1))>"
    );;
    assert!(((out as i64) == unimplemented!()));
    return true;
}
pub fn do_test_54(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_2(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_2(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_2(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_2(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_2(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_2(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_2(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_2(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_3(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_3(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_4(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_5(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_4(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_5(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_4(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_5(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_4(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_5(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_55(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_6(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_6(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_6(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_6(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_6(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_6(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_6(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_6(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_7(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_7(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_8(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_9(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_8(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_9(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_8(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_9(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_8(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_9(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_56(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_10(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_10(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_10(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_10(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_10(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_10(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_10(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_10(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_11(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_11(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_12(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_13(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_12(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_13(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_12(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_13(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_12(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_13(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_57(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_14(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_14(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_14(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_14(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_14(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_14(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_14(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_14(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_15(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_15(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_16(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_17(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_16(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_17(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_16(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_17(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_16(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_17(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_58(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_18(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_18(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_18(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_18(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_18(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_18(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_18(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_18(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_19(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_19(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_20(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_21(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_20(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_21(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_20(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_21(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_20(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_21(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_59(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_22(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_22(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_22(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_22(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_23(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_23(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_23(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_23(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_24(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_25(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_26(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_27(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_26(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_27(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_28(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_29(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_28(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_29(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_60(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_23(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_23(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_23(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_23(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_22(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_22(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_22(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_22(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_25(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_24(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_28(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_29(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_28(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_29(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_26(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_27(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_26(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_27(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_61(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_30(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_30(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_30(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_30(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_31(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_31(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_31(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_31(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_32(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_33(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_34(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_35(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_34(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_35(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_36(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_37(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_36(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_37(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_62(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_31(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_31(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_31(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_31(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_30(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_30(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_30(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_30(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_33(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_32(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_36(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_37(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_36(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_37(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_34(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_35(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_34(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_35(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_63(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_38(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_38(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_38(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_38(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_39(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_39(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_39(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_39(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_40(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_41(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_42(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_43(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_42(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_43(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_44(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_45(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_44(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_45(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_64(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_39(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_39(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_39(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_39(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_38(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_38(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_38(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_38(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_41(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_40(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_44(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_45(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_44(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_45(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_42(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_43(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_42(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_43(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_65(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_46(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_46(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_46(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_46(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_47(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_47(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_47(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_47(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_48(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_49(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_50(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_51(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_50(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_51(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_52(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_53(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_52(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_53(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_66(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((8_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_47(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_47(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_47(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_47(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_46(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_46(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_46(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_46(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_49(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_48(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_52(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_53(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_52(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_53(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_50(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_51(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_50(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_51(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn test_limits_67() -> bool {
    assert!((unimplemented!() == (u32::MAX as u32).wrapping_sub(1_u32)));
    assert!((unimplemented!() == u32::MAX));
    return true;
}
pub fn test_limits_68() -> bool {
    assert!((unimplemented!() == (u64::MAX as u64).wrapping_sub(1_u64)));
    assert!((unimplemented!() == u64::MAX));
    return true;
}
pub fn test_limits_69() -> bool {
    assert!((unimplemented!() == (i32::MAX - 1)));
    assert!((unimplemented!() == i32::MAX));
    return true;
}
pub fn test_limits_70() -> bool {
    assert!((unimplemented!() == (i64::MAX - 1_i64)));
    assert!((unimplemented!() == i64::MAX));
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
fn main_0(argc: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    let mut non_cce: i32 = argc;
    const _: () = assert!(true, "do_test<signed char>()");;
    const _: () = assert!(true, "do_test<short>()");;
    const _: () = assert!(true, "do_test<int>()");;
    const _: () = assert!(true, "do_test<long>()");;
    const _: () = assert!(true, "do_test<long long>()");;
    assert!(({ do_test_54(Some(non_cce),) }));
    assert!(({ do_test_55(Some(non_cce),) }));
    assert!(({ do_test_56(Some(non_cce),) }));
    assert!(({ do_test_57(Some(non_cce),) }));
    assert!(({ do_test_58(Some(non_cce),) }));
    const _: () = assert!(true, "do_test<std::int8_t>()");;
    const _: () = assert!(true, "do_test<std::int16_t>()");;
    const _: () = assert!(true, "do_test<std::int32_t>()");;
    const _: () = assert!(true, "do_test<std::int64_t>()");;
    assert!(({ do_test_54(Some(non_cce),) }));
    assert!(({ do_test_55(Some(non_cce),) }));
    assert!(({ do_test_56(Some(non_cce),) }));
    assert!(({ do_test_57(Some(non_cce),) }));
    const _: () = assert!(true, "do_test<signed char, int>()");;
    const _: () = assert!(true, "do_test<int, signed char>()");;
    const _: () = assert!(true, "do_test<short, int>()");;
    const _: () = assert!(true, "do_test<int, short>()");;
    const _: () = assert!(true, "do_test<int, long>()");;
    const _: () = assert!(true, "do_test<long, int>()");;
    const _: () = assert!(true, "do_test<int, long long>()");;
    const _: () = assert!(true, "do_test<long long, int>()");;
    assert!(({ do_test_59(Some(non_cce),) }));
    assert!(({ do_test_60(Some(non_cce),) }));
    assert!(({ do_test_61(Some(non_cce),) }));
    assert!(({ do_test_62(Some(non_cce),) }));
    assert!(({ do_test_63(Some(non_cce),) }));
    assert!(({ do_test_64(Some(non_cce),) }));
    assert!(({ do_test_65(Some(non_cce),) }));
    assert!(({ do_test_66(Some(non_cce),) }));
    {
        let res1: Value<i64> = Rc::new(RefCell::new(unimplemented!()));
        &(unimplemented!());
        const _: () = assert!(true, "std::is_same_v<decltype(res1), std::int64_t>");;
        assert!(((*res1.borrow()) == 1324997410816_i64));
    }
    {
        assert!(({ test_limits_67() }));
        assert!(({ test_limits_67() }));
        assert!(({ test_limits_68() }));
        assert!(({ test_limits_69() }));
        assert!(({ test_limits_69() }));
        assert!(({ test_limits_70() }));
        const _: () = assert!(true, "test_limits<unsigned int>()");;
        const _: () = assert!(true, "test_limits<std::uint32_t>()");;
        const _: () = assert!(true, "test_limits<std::uint64_t>()");;
        const _: () = assert!(true, "test_limits<int>()");;
        const _: () = assert!(true, "test_limits<std::int32_t>()");;
        const _: () = assert!(true, "test_limits<std::int64_t>()");;
    }
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = Cases_1.with(|_| ());
}
