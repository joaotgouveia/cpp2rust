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
    pub x: i32,
    pub y: i32,
    pub expect: i32,
}
pub static mut Cases_1: std::cell::LazyCell<[anon_0; 12]> = std::cell::LazyCell::new(|| unsafe {
    [
        anon_0 {
            x: 0,
            y: 0,
            expect: 0,
        },
        anon_0 {
            x: 1,
            y: 0,
            expect: 1,
        },
        anon_0 {
            x: 0,
            y: 1,
            expect: 1,
        },
        anon_0 {
            x: 1,
            y: 1,
            expect: 1,
        },
        anon_0 {
            x: 2,
            y: 3,
            expect: 1,
        },
        anon_0 {
            x: 2,
            y: 4,
            expect: 2,
        },
        anon_0 {
            x: 11,
            y: 9,
            expect: 1,
        },
        anon_0 {
            x: 36,
            y: 17,
            expect: 1,
        },
        anon_0 {
            x: 36,
            y: 18,
            expect: 18,
        },
        anon_0 {
            x: 25,
            y: 30,
            expect: 5,
        },
        anon_0 {
            x: 24,
            y: 16,
            expect: 8,
        },
        anon_0 {
            x: 124,
            y: 100,
            expect: 4,
        },
    ]
});
pub unsafe fn test0_2(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i8 = (in1 as i8);
    let mut value2: i8 = (in2 as i8);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!((((out as i8) as i32) == (unimplemented!() as i32)));
    return true;
}
pub unsafe fn test0_3(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u8 = (in1 as u8);
    let mut value2: u8 = (in2 as u8);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!((((out as u8) as i32) == (unimplemented!() as i32)));
    return true;
}
pub unsafe fn test0_4(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i8 = (in1 as i8);
    let mut value2: u8 = (in2 as u8);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_5(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u8 = (in1 as u8);
    let mut value2: i8 = (in2 as i8);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_6(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i16 = (in1 as i16);
    let mut value2: i16 = (in2 as i16);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!((((out as i16) as i32) == (unimplemented!() as i32)));
    return true;
}
pub unsafe fn test0_7(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u16 = (in1 as u16);
    let mut value2: u16 = (in2 as u16);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!((((out as u16) as i32) == (unimplemented!() as i32)));
    return true;
}
pub unsafe fn test0_8(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i16 = (in1 as i16);
    let mut value2: u16 = (in2 as u16);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_9(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u16 = (in1 as u16);
    let mut value2: i16 = (in2 as i16);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_10(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i32 = (in1 as i32);
    let mut value2: i32 = (in2 as i32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_11(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u32 = (in1 as u32);
    let mut value2: u32 = (in2 as u32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_12(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i32 = (in1 as i32);
    let mut value2: u32 = (in2 as u32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_13(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u32 = (in1 as u32);
    let mut value2: i32 = (in2 as i32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_14(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i64 = (in1 as i64);
    let mut value2: i64 = (in2 as i64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_15(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u64 = (in1 as u64);
    let mut value2: u64 = (in2 as u64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_16(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i64 = (in1 as i64);
    let mut value2: u64 = (in2 as u64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_17(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u64 = (in1 as u64);
    let mut value2: i64 = (in2 as i64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_18(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i64 = (in1 as i64);
    let mut value2: i64 = (in2 as i64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_19(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u64 = (in1 as u64);
    let mut value2: u64 = (in2 as u64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_20(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i64 = (in1 as i64);
    let mut value2: u64 = (in2 as u64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_21(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u64 = (in1 as u64);
    let mut value2: i64 = (in2 as i64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_22(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i8 = (in1 as i8);
    let mut value2: i32 = (in2 as i32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_23(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i32 = (in1 as i32);
    let mut value2: i8 = (in2 as i8);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_24(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u8 = (in1 as u8);
    let mut value2: u32 = (in2 as u32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_25(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u32 = (in1 as u32);
    let mut value2: u8 = (in2 as u8);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_26(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i8 = (in1 as i8);
    let mut value2: u32 = (in2 as u32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_27(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u32 = (in1 as u32);
    let mut value2: i8 = (in2 as i8);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_28(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i32 = (in1 as i32);
    let mut value2: u8 = (in2 as u8);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_29(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u8 = (in1 as u8);
    let mut value2: i32 = (in2 as i32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_30(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i16 = (in1 as i16);
    let mut value2: i32 = (in2 as i32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_31(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i32 = (in1 as i32);
    let mut value2: i16 = (in2 as i16);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_32(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u16 = (in1 as u16);
    let mut value2: u32 = (in2 as u32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_33(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u32 = (in1 as u32);
    let mut value2: u16 = (in2 as u16);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_34(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i16 = (in1 as i16);
    let mut value2: u32 = (in2 as u32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_35(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u32 = (in1 as u32);
    let mut value2: i16 = (in2 as i16);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_36(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i32 = (in1 as i32);
    let mut value2: u16 = (in2 as u16);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_37(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u16 = (in1 as u16);
    let mut value2: i32 = (in2 as i32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i32) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_38(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i32 = (in1 as i32);
    let mut value2: i64 = (in2 as i64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_39(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i64 = (in1 as i64);
    let mut value2: i32 = (in2 as i32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_40(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u32 = (in1 as u32);
    let mut value2: u64 = (in2 as u64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_41(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u64 = (in1 as u64);
    let mut value2: u32 = (in2 as u32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_42(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i32 = (in1 as i32);
    let mut value2: u64 = (in2 as u64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_43(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u64 = (in1 as u64);
    let mut value2: i32 = (in2 as i32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_44(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i64 = (in1 as i64);
    let mut value2: u32 = (in2 as u32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_45(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u32 = (in1 as u32);
    let mut value2: i64 = (in2 as i64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_46(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i32 = (in1 as i32);
    let mut value2: i64 = (in2 as i64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_47(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i64 = (in1 as i64);
    let mut value2: i32 = (in2 as i32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_48(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u32 = (in1 as u32);
    let mut value2: u64 = (in2 as u64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_49(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u64 = (in1 as u64);
    let mut value2: u32 = (in2 as u32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_50(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i32 = (in1 as i32);
    let mut value2: u64 = (in2 as u64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_51(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u64 = (in1 as u64);
    let mut value2: i32 = (in2 as i32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as u64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_52(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: i64 = (in1 as i64);
    let mut value2: u32 = (in2 as u32);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i64) == (unimplemented!())));
    return true;
}
pub unsafe fn test0_53(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
    let mut value1: u32 = (in1 as u32);
    let mut value2: i64 = (in2 as i64);
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value1, value2))>"
    );;
    const _: () = assert!(
        true,
        "std::is_same_v<Output, decltype(std::gcd(value2, value1))>"
    );;
    assert!(((out as i64) == (unimplemented!())));
    return true;
}
pub unsafe fn basic_gcd__54(mut m: u8, mut n: u8) -> u8 {
    return if ((n as i32) == (0)) {
        m
    } else {
        (unsafe {
            let _m: u8 = n;
            let _n: u8 = (((m as i32) % (n as i32)) as u8);
            basic_gcd__54(_m, _n)
        })
    };
}
pub unsafe fn basic_gcd__55(mut m: u16, mut n: u16) -> u16 {
    return if ((n as i32) == (0)) {
        m
    } else {
        (unsafe {
            let _m: u16 = n;
            let _n: u16 = (((m as i32) % (n as i32)) as u16);
            basic_gcd__55(_m, _n)
        })
    };
}
pub unsafe fn basic_gcd__56(mut m: u32, mut n: u32) -> u32 {
    return if ((n) == (0_u32)) {
        m
    } else {
        (unsafe {
            let _m: u32 = n;
            let _n: u32 = (m).wrapping_rem(n);
            basic_gcd__56(_m, _n)
        })
    };
}
pub unsafe fn basic_gcd__57(mut m: u64, mut n: u64) -> u64 {
    return if ((n) == (0_u64)) {
        m
    } else {
        (unsafe {
            let _m: u64 = n;
            let _n: u64 = (m).wrapping_rem(n);
            basic_gcd__57(_m, _n)
        })
    };
}
pub unsafe fn basic_gcd_58(mut m: i8, mut n: i8) -> i8 {
    {
        if ((m as i32) < (0)) && ((m as i32) != (i8::MIN as i32)) {
            m = (-(m as i32) as i8);
        }
        if ((n as i32) < (0)) && ((n as i32) != (i8::MIN as i32)) {
            n = (-(n as i32) as i8);
        }
    }
    return ((unsafe { basic_gcd__54(((m as ()) as u8), ((n as ()) as u8)) }) as i8);
}
pub unsafe fn basic_gcd_59(mut m: i16, mut n: i16) -> i16 {
    {
        if ((m as i32) < (0)) && ((m as i32) != (i16::MIN as i32)) {
            m = (-(m as i32) as i16);
        }
        if ((n as i32) < (0)) && ((n as i32) != (i16::MIN as i32)) {
            n = (-(n as i32) as i16);
        }
    }
    return ((unsafe { basic_gcd__55(((m as ()) as u16), ((n as ()) as u16)) }) as i16);
}
pub unsafe fn basic_gcd_60(mut m: i32, mut n: i32) -> i32 {
    {
        if ((m) < (0)) && ((m) != (i32::MIN)) {
            m = -m;
        }
        if ((n) < (0)) && ((n) != (i32::MIN)) {
            n = -n;
        }
    }
    return ((unsafe { basic_gcd__56(((m as ()) as u32), ((n as ()) as u32)) }) as i32);
}
pub unsafe fn basic_gcd_61(mut m: i64, mut n: i64) -> i64 {
    {
        if ((m) < (0_i64)) && ((m) != (i64::MIN)) {
            m = -m;
        }
        if ((n) < (0_i64)) && ((n) != (i64::MIN)) {
            n = -n;
        }
    }
    return ((unsafe { basic_gcd__57(((m as ()) as u64), ((n as ()) as u64)) }) as i64);
}
pub unsafe fn basic_gcd_62(mut m: u8, mut n: u8) -> u8 {
    {}
    return (unsafe { basic_gcd__54(((m as ()) as u8), ((n as ()) as u8)) });
}
pub unsafe fn basic_gcd_63(mut m: u16, mut n: u16) -> u16 {
    {}
    return (unsafe { basic_gcd__55(((m as ()) as u16), ((n as ()) as u16)) });
}
pub unsafe fn basic_gcd_64(mut m: u32, mut n: u32) -> u32 {
    {}
    return (unsafe { basic_gcd__56(((m as ()) as u32), ((n as ()) as u32)) });
}
pub unsafe fn basic_gcd_65(mut m: u64, mut n: u64) -> u64 {
    {}
    return (unsafe { basic_gcd__57(((m as ()) as u64), ((n as ()) as u64)) });
}
pub unsafe fn do_fuzzy_tests_66() {
    let mut gen_: () = unimplemented!();
    let max_input: i8 = i8::MAX;
    let mut distrib: () = unimplemented!();
    let nb_rounds: i32 = 10000;
    let mut i: i32 = 0;
    'loop_: while ((i) < (nb_rounds)) {
        let mut n: i8 = (unimplemented!() as i8);
        let mut m: i8 = (unimplemented!() as i8);
        assert!(((unimplemented!() as i32) == ((unsafe { basic_gcd_58(n, m,) }) as i32)));
        i.prefix_inc();
    }
}
pub unsafe fn do_fuzzy_tests_67() {
    let mut gen_: () = unimplemented!();
    let max_input: i16 = i16::MAX;
    let mut distrib: () = unimplemented!();
    let nb_rounds: i32 = 10000;
    let mut i: i32 = 0;
    'loop_: while ((i) < (nb_rounds)) {
        let mut n: i16 = (unimplemented!() as i16);
        let mut m: i16 = (unimplemented!() as i16);
        assert!(((unimplemented!() as i32) == ((unsafe { basic_gcd_59(n, m,) }) as i32)));
        i.prefix_inc();
    }
}
pub unsafe fn do_fuzzy_tests_68() {
    let mut gen_: () = unimplemented!();
    let max_input: i32 = i32::MAX;
    let mut distrib: () = unimplemented!();
    let nb_rounds: i32 = 10000;
    let mut i: i32 = 0;
    'loop_: while ((i) < (nb_rounds)) {
        let mut n: i32 = (unimplemented!() as i32);
        let mut m: i32 = (unimplemented!() as i32);
        assert!(((unimplemented!()) == (unsafe { basic_gcd_60(n, m,) })));
        i.prefix_inc();
    }
}
pub unsafe fn do_fuzzy_tests_69() {
    let mut gen_: () = unimplemented!();
    let max_input: i64 = i64::MAX;
    let mut distrib: () = unimplemented!();
    let nb_rounds: i32 = 10000;
    let mut i: i32 = 0;
    'loop_: while ((i) < (nb_rounds)) {
        let mut n: i64 = (unimplemented!() as i64);
        let mut m: i64 = (unimplemented!() as i64);
        assert!(((unimplemented!()) == (unsafe { basic_gcd_61(n, m,) })));
        i.prefix_inc();
    }
}
pub unsafe fn do_fuzzy_tests_70() {
    let mut gen_: () = unimplemented!();
    let max_input: u8 = u8::MAX;
    let mut distrib: () = unimplemented!();
    let nb_rounds: i32 = 10000;
    let mut i: i32 = 0;
    'loop_: while ((i) < (nb_rounds)) {
        let mut n: u8 = (unimplemented!() as u8);
        let mut m: u8 = (unimplemented!() as u8);
        assert!(((unimplemented!() as i32) == ((unsafe { basic_gcd_62(n, m,) }) as i32)));
        i.prefix_inc();
    }
}
pub unsafe fn do_fuzzy_tests_71() {
    let mut gen_: () = unimplemented!();
    let max_input: u16 = u16::MAX;
    let mut distrib: () = unimplemented!();
    let nb_rounds: i32 = 10000;
    let mut i: i32 = 0;
    'loop_: while ((i) < (nb_rounds)) {
        let mut n: u16 = (unimplemented!() as u16);
        let mut m: u16 = (unimplemented!() as u16);
        assert!(((unimplemented!() as i32) == ((unsafe { basic_gcd_63(n, m,) }) as i32)));
        i.prefix_inc();
    }
}
pub unsafe fn do_fuzzy_tests_72() {
    let mut gen_: () = unimplemented!();
    let max_input: u32 = u32::MAX;
    let mut distrib: () = unimplemented!();
    let nb_rounds: i32 = 10000;
    let mut i: i32 = 0;
    'loop_: while ((i) < (nb_rounds)) {
        let mut n: u32 = (unimplemented!() as u32);
        let mut m: u32 = (unimplemented!() as u32);
        assert!(((unimplemented!()) == (unsafe { basic_gcd_64(n, m,) })));
        i.prefix_inc();
    }
}
pub unsafe fn do_fuzzy_tests_73() {
    let mut gen_: () = unimplemented!();
    let max_input: u64 = u64::MAX;
    let mut distrib: () = unimplemented!();
    let nb_rounds: i32 = 10000;
    let mut i: i32 = 0;
    'loop_: while ((i) < (nb_rounds)) {
        let mut n: u64 = (unimplemented!() as u64);
        let mut m: u64 = (unimplemented!() as u64);
        assert!(((unimplemented!()) == (unsafe { basic_gcd_65(n, m,) })));
        i.prefix_inc();
    }
}
pub unsafe fn do_limit_tests_74() {
    let mut inputs: [i8; 27] = [
        (((i8::MIN as i32) + (if unimplemented!() { 3 } else { 0 })) as i8),
        (((i8::MIN as i32) + (1)) as i8),
        (((i8::MIN as i32) + (2)) as i8),
        i8::MAX,
        (((i8::MAX as i32) - (1)) as i8),
        (((i8::MAX as i32) - (2)) as i8),
        0_i8,
        1_i8,
        2_i8,
        3_i8,
        4_i8,
        5_i8,
        6_i8,
        7_i8,
        8_i8,
        9_i8,
        10_i8,
        (-1_i32 as i8),
        (-2_i32 as i8),
        (-3_i32 as i8),
        (-4_i32 as i8),
        (-5_i32 as i8),
        (-6_i32 as i8),
        (-7_i32 as i8),
        (-8_i32 as i8),
        (-9_i32 as i8),
        (-10_i32 as i8),
    ];
    'loop_: for n in IterRange::new(
        inputs.as_mut_ptr(),
        inputs.as_mut_ptr().offset((27_i64) as isize),
    ) {
        let mut n: i8 = (*n);
        'loop_: for m in IterRange::new(
            inputs.as_mut_ptr(),
            inputs.as_mut_ptr().offset((27_i64) as isize),
        ) {
            let mut m: i8 = (*m);
            assert!(((unimplemented!() as i32) == ((unsafe { basic_gcd_58(n, m,) }) as i32)));
        }
    }
}
pub unsafe fn do_limit_tests_75() {
    let mut inputs: [i16; 27] = [
        (((i16::MIN as i32) + (if unimplemented!() { 3 } else { 0 })) as i16),
        (((i16::MIN as i32) + (1)) as i16),
        (((i16::MIN as i32) + (2)) as i16),
        i16::MAX,
        (((i16::MAX as i32) - (1)) as i16),
        (((i16::MAX as i32) - (2)) as i16),
        0_i16,
        1_i16,
        2_i16,
        3_i16,
        4_i16,
        5_i16,
        6_i16,
        7_i16,
        8_i16,
        9_i16,
        10_i16,
        (-1_i32 as i16),
        (-2_i32 as i16),
        (-3_i32 as i16),
        (-4_i32 as i16),
        (-5_i32 as i16),
        (-6_i32 as i16),
        (-7_i32 as i16),
        (-8_i32 as i16),
        (-9_i32 as i16),
        (-10_i32 as i16),
    ];
    'loop_: for n in IterRange::new(
        inputs.as_mut_ptr(),
        inputs.as_mut_ptr().offset((27_i64) as isize),
    ) {
        let mut n: i16 = (*n);
        'loop_: for m in IterRange::new(
            inputs.as_mut_ptr(),
            inputs.as_mut_ptr().offset((27_i64) as isize),
        ) {
            let mut m: i16 = (*m);
            assert!(((unimplemented!() as i32) == ((unsafe { basic_gcd_59(n, m,) }) as i32)));
        }
    }
}
pub unsafe fn do_limit_tests_76() {
    let mut inputs: [i32; 27] = [
        ((i32::MIN) + (if unimplemented!() { 3 } else { 0 })),
        ((i32::MIN) + (1)),
        ((i32::MIN) + (2)),
        i32::MAX,
        ((i32::MAX) - (1)),
        ((i32::MAX) - (2)),
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        9,
        10,
        (-1_i32 as i32),
        (-2_i32 as i32),
        (-3_i32 as i32),
        (-4_i32 as i32),
        (-5_i32 as i32),
        (-6_i32 as i32),
        (-7_i32 as i32),
        (-8_i32 as i32),
        (-9_i32 as i32),
        (-10_i32 as i32),
    ];
    'loop_: for n in IterRange::new(
        inputs.as_mut_ptr(),
        inputs.as_mut_ptr().offset((27_i64) as isize),
    ) {
        let mut n: i32 = (*n);
        'loop_: for m in IterRange::new(
            inputs.as_mut_ptr(),
            inputs.as_mut_ptr().offset((27_i64) as isize),
        ) {
            let mut m: i32 = (*m);
            assert!(((unimplemented!()) == (unsafe { basic_gcd_60(n, m,) })));
        }
    }
}
pub unsafe fn do_limit_tests_77() {
    let mut inputs: [i64; 27] = [
        ((i64::MIN) + ((if unimplemented!() { 3 } else { 0 }) as i64)),
        ((i64::MIN) + (1_i64)),
        ((i64::MIN) + (2_i64)),
        i64::MAX,
        ((i64::MAX) - (1_i64)),
        ((i64::MAX) - (2_i64)),
        0_i64,
        1_i64,
        2_i64,
        3_i64,
        4_i64,
        5_i64,
        6_i64,
        7_i64,
        8_i64,
        9_i64,
        10_i64,
        (-1_i32 as i64),
        (-2_i32 as i64),
        (-3_i32 as i64),
        (-4_i32 as i64),
        (-5_i32 as i64),
        (-6_i32 as i64),
        (-7_i32 as i64),
        (-8_i32 as i64),
        (-9_i32 as i64),
        (-10_i32 as i64),
    ];
    'loop_: for n in IterRange::new(
        inputs.as_mut_ptr(),
        inputs.as_mut_ptr().offset((27_i64) as isize),
    ) {
        let mut n: i64 = (*n);
        'loop_: for m in IterRange::new(
            inputs.as_mut_ptr(),
            inputs.as_mut_ptr().offset((27_i64) as isize),
        ) {
            let mut m: i64 = (*m);
            assert!(((unimplemented!()) == (unsafe { basic_gcd_61(n, m,) })));
        }
    }
}
pub unsafe fn do_limit_tests_78() {
    let mut inputs: [u8; 27] = [
        (((u8::MIN as i32) + (if unimplemented!() { 3 } else { 0 })) as u8),
        (((u8::MIN as i32) + (1)) as u8),
        (((u8::MIN as i32) + (2)) as u8),
        u8::MAX,
        (((u8::MAX as i32) - (1)) as u8),
        (((u8::MAX as i32) - (2)) as u8),
        0_u8,
        1_u8,
        2_u8,
        3_u8,
        4_u8,
        5_u8,
        6_u8,
        7_u8,
        8_u8,
        9_u8,
        10_u8,
        (-1_i32 as u8),
        (-2_i32 as u8),
        (-3_i32 as u8),
        (-4_i32 as u8),
        (-5_i32 as u8),
        (-6_i32 as u8),
        (-7_i32 as u8),
        (-8_i32 as u8),
        (-9_i32 as u8),
        (-10_i32 as u8),
    ];
    'loop_: for n in IterRange::new(
        inputs.as_mut_ptr(),
        inputs.as_mut_ptr().offset((27_i64) as isize),
    ) {
        let mut n: u8 = (*n);
        'loop_: for m in IterRange::new(
            inputs.as_mut_ptr(),
            inputs.as_mut_ptr().offset((27_i64) as isize),
        ) {
            let mut m: u8 = (*m);
            assert!(((unimplemented!() as i32) == ((unsafe { basic_gcd_62(n, m,) }) as i32)));
        }
    }
}
pub unsafe fn do_limit_tests_79() {
    let mut inputs: [u16; 27] = [
        (((u16::MIN as i32) + (if unimplemented!() { 3 } else { 0 })) as u16),
        (((u16::MIN as i32) + (1)) as u16),
        (((u16::MIN as i32) + (2)) as u16),
        u16::MAX,
        (((u16::MAX as i32) - (1)) as u16),
        (((u16::MAX as i32) - (2)) as u16),
        0_u16,
        1_u16,
        2_u16,
        3_u16,
        4_u16,
        5_u16,
        6_u16,
        7_u16,
        8_u16,
        9_u16,
        10_u16,
        (-1_i32 as u16),
        (-2_i32 as u16),
        (-3_i32 as u16),
        (-4_i32 as u16),
        (-5_i32 as u16),
        (-6_i32 as u16),
        (-7_i32 as u16),
        (-8_i32 as u16),
        (-9_i32 as u16),
        (-10_i32 as u16),
    ];
    'loop_: for n in IterRange::new(
        inputs.as_mut_ptr(),
        inputs.as_mut_ptr().offset((27_i64) as isize),
    ) {
        let mut n: u16 = (*n);
        'loop_: for m in IterRange::new(
            inputs.as_mut_ptr(),
            inputs.as_mut_ptr().offset((27_i64) as isize),
        ) {
            let mut m: u16 = (*m);
            assert!(((unimplemented!() as i32) == ((unsafe { basic_gcd_63(n, m,) }) as i32)));
        }
    }
}
pub unsafe fn do_limit_tests_80() {
    let mut inputs: [u32; 27] = [
        (u32::MIN as u32).wrapping_add(((if unimplemented!() { 3 } else { 0 }) as u32)),
        (u32::MIN as u32).wrapping_add(1_u32),
        (u32::MIN as u32).wrapping_add(2_u32),
        u32::MAX,
        (u32::MAX as u32).wrapping_sub(1_u32),
        (u32::MAX as u32).wrapping_sub(2_u32),
        0_u32,
        1_u32,
        2_u32,
        3_u32,
        4_u32,
        5_u32,
        6_u32,
        7_u32,
        8_u32,
        9_u32,
        10_u32,
        (-1_i32 as u32),
        (-2_i32 as u32),
        (-3_i32 as u32),
        (-4_i32 as u32),
        (-5_i32 as u32),
        (-6_i32 as u32),
        (-7_i32 as u32),
        (-8_i32 as u32),
        (-9_i32 as u32),
        (-10_i32 as u32),
    ];
    'loop_: for n in IterRange::new(
        inputs.as_mut_ptr(),
        inputs.as_mut_ptr().offset((27_i64) as isize),
    ) {
        let mut n: u32 = (*n);
        'loop_: for m in IterRange::new(
            inputs.as_mut_ptr(),
            inputs.as_mut_ptr().offset((27_i64) as isize),
        ) {
            let mut m: u32 = (*m);
            assert!(((unimplemented!()) == (unsafe { basic_gcd_64(n, m,) })));
        }
    }
}
pub unsafe fn do_limit_tests_81() {
    let mut inputs: [u64; 27] = [
        (u64::MIN as u64).wrapping_add(((if unimplemented!() { 3 } else { 0 }) as u64)),
        (u64::MIN as u64).wrapping_add(1_u64),
        (u64::MIN as u64).wrapping_add(2_u64),
        u64::MAX,
        (u64::MAX as u64).wrapping_sub(1_u64),
        (u64::MAX as u64).wrapping_sub(2_u64),
        0_u64,
        1_u64,
        2_u64,
        3_u64,
        4_u64,
        5_u64,
        6_u64,
        7_u64,
        8_u64,
        9_u64,
        10_u64,
        (-1_i32 as u64),
        (-2_i32 as u64),
        (-3_i32 as u64),
        (-4_i32 as u64),
        (-5_i32 as u64),
        (-6_i32 as u64),
        (-7_i32 as u64),
        (-8_i32 as u64),
        (-9_i32 as u64),
        (-10_i32 as u64),
    ];
    'loop_: for n in IterRange::new(
        inputs.as_mut_ptr(),
        inputs.as_mut_ptr().offset((27_i64) as isize),
    ) {
        let mut n: u64 = (*n);
        'loop_: for m in IterRange::new(
            inputs.as_mut_ptr(),
            inputs.as_mut_ptr().offset((27_i64) as isize),
        ) {
            let mut m: u64 = (*m);
            assert!(((unimplemented!()) == (unsafe { basic_gcd_65(n, m,) })));
        }
    }
}
pub unsafe fn do_test_82(mut _a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| unsafe { 0 });
    let mut accumulate: bool = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_2(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_2(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_2(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_2(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_2(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_2(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_2(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_2(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_3(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_3(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_4(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_5(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_4(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_5(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_4(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_5(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_4(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_5(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
    }
    return accumulate;
}
pub unsafe fn do_test_83(mut _a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| unsafe { 0 });
    let mut accumulate: bool = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_6(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_6(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_6(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_6(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_6(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_6(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_6(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_6(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_7(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_7(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_8(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_9(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_8(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_9(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_8(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_9(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_8(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_9(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
    }
    return accumulate;
}
pub unsafe fn do_test_84(mut _a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| unsafe { 0 });
    let mut accumulate: bool = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_10(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_10(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_10(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_10(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_10(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_10(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_10(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_10(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_11(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_11(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_12(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_13(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_12(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_13(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_12(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_13(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_12(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_13(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
    }
    return accumulate;
}
pub unsafe fn do_test_85(mut _a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| unsafe { 0 });
    let mut accumulate: bool = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_14(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_14(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_14(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_14(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_14(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_14(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_14(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_14(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_15(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_15(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_16(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_17(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_16(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_17(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_16(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_17(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_16(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_17(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
    }
    return accumulate;
}
pub unsafe fn do_test_86(mut _a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| unsafe { 0 });
    let mut accumulate: bool = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_18(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_18(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_18(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_18(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_18(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_18(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_18(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_18(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_19(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_19(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_20(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_21(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_20(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_21(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_20(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_21(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_20(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_21(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
    }
    return accumulate;
}
pub unsafe fn do_test_87(mut _a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| unsafe { 0 });
    let mut accumulate: bool = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_22(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_22(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_22(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_22(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_23(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_23(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_23(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_23(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_24(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_25(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_26(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_27(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_26(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_27(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_28(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_29(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_28(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_29(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
    }
    return accumulate;
}
pub unsafe fn do_test_88(mut _a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| unsafe { 0 });
    let mut accumulate: bool = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_23(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_23(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_23(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_23(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_22(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_22(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_22(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_22(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_25(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_24(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_28(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_29(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_28(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_29(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_26(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_27(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_26(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_27(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
    }
    return accumulate;
}
pub unsafe fn do_test_89(mut _a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| unsafe { 0 });
    let mut accumulate: bool = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_30(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_30(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_30(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_30(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_31(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_31(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_31(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_31(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_32(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_33(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_34(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_35(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_34(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_35(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_36(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_37(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_36(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_37(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
    }
    return accumulate;
}
pub unsafe fn do_test_90(mut _a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| unsafe { 0 });
    let mut accumulate: bool = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_31(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_31(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_31(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_31(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_30(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_30(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_30(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_30(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_33(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_32(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_36(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_37(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_36(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_37(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_34(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_35(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_34(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_35(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
    }
    return accumulate;
}
pub unsafe fn do_test_91(mut _a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| unsafe { 0 });
    let mut accumulate: bool = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_38(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_38(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_38(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_38(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_39(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_39(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_39(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_39(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_40(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_41(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_42(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_43(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_42(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_43(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_44(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_45(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_44(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_45(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
    }
    return accumulate;
}
pub unsafe fn do_test_92(mut _a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| unsafe { 0 });
    let mut accumulate: bool = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_39(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_39(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_39(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_39(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_38(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_38(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_38(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_38(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_41(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_40(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_44(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_45(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_44(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_45(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_42(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_43(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_42(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_43(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
    }
    return accumulate;
}
pub unsafe fn do_test_93(mut _a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| unsafe { 0 });
    let mut accumulate: bool = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_46(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_46(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_46(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_46(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_47(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_47(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_47(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_47(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_48(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_49(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_50(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_51(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_50(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_51(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_52(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_53(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_52(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_53(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
    }
    return accumulate;
}
pub unsafe fn do_test_94(mut _a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| unsafe { 0 });
    let mut accumulate: bool = true;
    'loop_: for TC in IterRange::new(
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1)).as_ptr(),
        (*std::cell::LazyCell::force_mut(&mut *&raw mut Cases_1))
            .as_ptr()
            .offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC);
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_47(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_47(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_47(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_47(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_46(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_46(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_46(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_46(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_49(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_48(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_52(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_53(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_52(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_53(_in1, _in2, _out)
                }) as i32))
                != 0;
        }
        {
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_50(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_51(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = -TC.x;
                    let _in2: i32 = TC.y;
                    let _out: i32 = TC.expect;
                    test0_50(_in1, _in2, _out)
                }) as i32))
                != 0;
            accumulate = ((accumulate as i32)
                & ((unsafe {
                    let _in1: i32 = TC.x;
                    let _in2: i32 = -TC.y;
                    let _out: i32 = TC.expect;
                    test0_51(_in1, _in2, _out)
                }) as i32))
                != 0;
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
unsafe fn main_0(mut argc: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    let mut non_cce: i32 = argc;
    const _: () = assert!(true, "do_test<signed char>()");;
    const _: () = assert!(true, "do_test<short>()");;
    const _: () = assert!(true, "do_test<int>()");;
    const _: () = assert!(true, "do_test<long>()");;
    const _: () = assert!(true, "do_test<long long>()");;
    assert!((unsafe { do_test_82(Some(non_cce),) }));
    assert!((unsafe { do_test_83(Some(non_cce),) }));
    assert!((unsafe { do_test_84(Some(non_cce),) }));
    assert!((unsafe { do_test_85(Some(non_cce),) }));
    assert!((unsafe { do_test_86(Some(non_cce),) }));
    const _: () = assert!(true, "do_test<std::int8_t>()");;
    const _: () = assert!(true, "do_test<std::int16_t>()");;
    const _: () = assert!(true, "do_test<std::int32_t>()");;
    const _: () = assert!(true, "do_test<std::int64_t>()");;
    assert!((unsafe { do_test_82(Some(non_cce),) }));
    assert!((unsafe { do_test_83(Some(non_cce),) }));
    assert!((unsafe { do_test_84(Some(non_cce),) }));
    assert!((unsafe { do_test_85(Some(non_cce),) }));
    const _: () = assert!(true, "do_test<signed char, int>()");;
    const _: () = assert!(true, "do_test<int, signed char>()");;
    const _: () = assert!(true, "do_test<short, int>()");;
    const _: () = assert!(true, "do_test<int, short>()");;
    const _: () = assert!(true, "do_test<int, long>()");;
    const _: () = assert!(true, "do_test<long, int>()");;
    const _: () = assert!(true, "do_test<int, long long>()");;
    const _: () = assert!(true, "do_test<long long, int>()");;
    assert!((unsafe { do_test_87(Some(non_cce),) }));
    assert!((unsafe { do_test_88(Some(non_cce),) }));
    assert!((unsafe { do_test_89(Some(non_cce),) }));
    assert!((unsafe { do_test_90(Some(non_cce),) }));
    assert!((unsafe { do_test_91(Some(non_cce),) }));
    assert!((unsafe { do_test_92(Some(non_cce),) }));
    assert!((unsafe { do_test_93(Some(non_cce),) }));
    assert!((unsafe { do_test_94(Some(non_cce),) }));
    {
        let mut res: i64 = unimplemented!();
        const _: () = assert!(true, "std::is_same_v<decltype(res), std::int64_t>");;
        assert!(((res) == (2_i64)));
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
    (unsafe { do_limit_tests_75() });
    (unsafe { do_limit_tests_76() });
    (unsafe { do_limit_tests_77() });
    (unsafe { do_limit_tests_78() });
    (unsafe { do_limit_tests_79() });
    (unsafe { do_limit_tests_80() });
    (unsafe { do_limit_tests_81() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const Cases_1);
}
