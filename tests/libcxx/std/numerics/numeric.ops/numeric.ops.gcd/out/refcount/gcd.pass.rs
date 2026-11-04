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
impl CppMove for anon_0 {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
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
    ])));
);
pub fn test0_2(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3((((out as i8) as i32) == (({ gcd_4(value1, value2) }) as i32))) });
    return true;
}
pub fn test0_5(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3((((out as u8) as i32) == (({ gcd_6(value1, value2) }) as i32))) });
    return true;
}
pub fn test0_7(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ gcd_8(value1, value2) }))) });
    return true;
}
pub fn test0_9(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ gcd_10(value1, value2) }))) });
    return true;
}
pub fn test0_11(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3((((out as i16) as i32) == (({ gcd_12(value1, value2) }) as i32))) });
    return true;
}
pub fn test0_13(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3((((out as u16) as i32) == (({ gcd_14(value1, value2) }) as i32))) });
    return true;
}
pub fn test0_15(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ gcd_16(value1, value2) }))) });
    return true;
}
pub fn test0_17(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ gcd_18(value1, value2) }))) });
    return true;
}
pub fn test0_19(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ gcd_20(value1, value2) }))) });
    return true;
}
pub fn test0_21(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ gcd_22(value1, value2) }))) });
    return true;
}
pub fn test0_23(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ gcd_24(value1, value2) }))) });
    return true;
}
pub fn test0_25(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ gcd_26(value1, value2) }))) });
    return true;
}
pub fn test0_27(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i64) == ({ gcd_28(value1, value2) }))) });
    return true;
}
pub fn test0_29(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ gcd_30(value1, value2) }))) });
    return true;
}
pub fn test0_31(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ gcd_32(value1, value2) }))) });
    return true;
}
pub fn test0_33(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ gcd_34(value1, value2) }))) });
    return true;
}
pub fn test0_35(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i64) == ({ gcd_36(value1, value2) }))) });
    return true;
}
pub fn test0_37(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ gcd_38(value1, value2) }))) });
    return true;
}
pub fn test0_39(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ gcd_40(value1, value2) }))) });
    return true;
}
pub fn test0_41(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ gcd_42(value1, value2) }))) });
    return true;
}
pub fn test0_43(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ gcd_44(value1, value2) }))) });
    return true;
}
pub fn test0_45(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ gcd_46(value1, value2) }))) });
    return true;
}
pub fn test0_47(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ gcd_48(value1, value2) }))) });
    return true;
}
pub fn test0_49(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ gcd_50(value1, value2) }))) });
    return true;
}
pub fn test0_51(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ gcd_52(value1, value2) }))) });
    return true;
}
pub fn test0_53(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ gcd_54(value1, value2) }))) });
    return true;
}
pub fn test0_55(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ gcd_56(value1, value2) }))) });
    return true;
}
pub fn test0_57(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ gcd_58(value1, value2) }))) });
    return true;
}
pub fn test0_59(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ gcd_60(value1, value2) }))) });
    return true;
}
pub fn test0_61(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ gcd_62(value1, value2) }))) });
    return true;
}
pub fn test0_63(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ gcd_64(value1, value2) }))) });
    return true;
}
pub fn test0_65(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ gcd_66(value1, value2) }))) });
    return true;
}
pub fn test0_67(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ gcd_68(value1, value2) }))) });
    return true;
}
pub fn test0_69(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ gcd_70(value1, value2) }))) });
    return true;
}
pub fn test0_71(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ gcd_72(value1, value2) }))) });
    return true;
}
pub fn test0_73(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ gcd_74(value1, value2) }))) });
    return true;
}
pub fn test0_75(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i64) == ({ gcd_76(value1, value2) }))) });
    return true;
}
pub fn test0_77(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i64) == ({ gcd_78(value1, value2) }))) });
    return true;
}
pub fn test0_79(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ gcd_80(value1, value2) }))) });
    return true;
}
pub fn test0_81(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ gcd_82(value1, value2) }))) });
    return true;
}
pub fn test0_83(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ gcd_84(value1, value2) }))) });
    return true;
}
pub fn test0_85(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ gcd_86(value1, value2) }))) });
    return true;
}
pub fn test0_87(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i64) == ({ gcd_88(value1, value2) }))) });
    return true;
}
pub fn test0_89(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i64) == ({ gcd_90(value1, value2) }))) });
    return true;
}
pub fn test0_91(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i64) == ({ gcd_92(value1, value2) }))) });
    return true;
}
pub fn test0_93(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i64) == ({ gcd_94(value1, value2) }))) });
    return true;
}
pub fn test0_95(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ gcd_96(value1, value2) }))) });
    return true;
}
pub fn test0_97(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ gcd_98(value1, value2) }))) });
    return true;
}
pub fn test0_99(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ gcd_100(value1, value2) }))) });
    return true;
}
pub fn test0_101(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ gcd_102(value1, value2) }))) });
    return true;
}
pub fn test0_103(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i64) == ({ gcd_104(value1, value2) }))) });
    return true;
}
pub fn test0_105(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i64) == ({ gcd_106(value1, value2) }))) });
    return true;
}
pub fn basic_gcd__107(mut m: u8, mut n: u8) -> u8 {
    return if ((n as i32) == 0) {
        m
    } else {
        ({
            let _m: u8 = n;
            let _n: u8 = (((m as i32) % (n as i32)) as u8);
            basic_gcd__107(_m, _n)
        })
    };
}
pub fn basic_gcd__108(mut m: u16, mut n: u16) -> u16 {
    return if ((n as i32) == 0) {
        m
    } else {
        ({
            let _m: u16 = n;
            let _n: u16 = (((m as i32) % (n as i32)) as u16);
            basic_gcd__108(_m, _n)
        })
    };
}
pub fn basic_gcd__109(mut m: u32, mut n: u32) -> u32 {
    return if (n == 0_u32) {
        m
    } else {
        ({
            let _m: u32 = n;
            let _n: u32 = (m).wrapping_rem(n);
            basic_gcd__109(_m, _n)
        })
    };
}
pub fn basic_gcd__110(mut m: u64, mut n: u64) -> u64 {
    return if (n == 0_u64) {
        m
    } else {
        ({
            let _m: u64 = n;
            let _n: u64 = (m).wrapping_rem(n);
            basic_gcd__110(_m, _n)
        })
    };
}
pub fn basic_gcd_111(mut m: i8, mut n: i8) -> i8 {
    {
        if ((m as i32) < 0) && ((m as i32) != (({ std_numeric_limits_signed_char_::min() }) as i32))
        {
            m = { (-(m as i32) as i8) };
        }
        if ((n as i32) < 0) && ((n as i32) != (({ std_numeric_limits_signed_char_::min() }) as i32))
        {
            n = { (-(n as i32) as i8) };
        }
    }
    return (({ basic_gcd__107((m as u8), (n as u8)) }) as i8);
}
pub fn basic_gcd_112(mut m: i16, mut n: i16) -> i16 {
    {
        if ((m as i32) < 0) && ((m as i32) != (({ std_numeric_limits_short_::min() }) as i32)) {
            m = { (-(m as i32) as i16) };
        }
        if ((n as i32) < 0) && ((n as i32) != (({ std_numeric_limits_short_::min() }) as i32)) {
            n = { (-(n as i32) as i16) };
        }
    }
    return (({ basic_gcd__108((m as u16), (n as u16)) }) as i16);
}
pub fn basic_gcd_113(mut m: i32, mut n: i32) -> i32 {
    {
        if (m < 0) && (m != ({ std_numeric_limits_int_::min() })) {
            m = { -m };
        }
        if (n < 0) && (n != ({ std_numeric_limits_int_::min() })) {
            n = { -n };
        }
    }
    return (({ basic_gcd__109((m as u32), (n as u32)) }) as i32);
}
pub fn basic_gcd_114(mut m: i64, mut n: i64) -> i64 {
    {
        if (m < 0_i64) && (m != ({ std_numeric_limits_long_::min() })) {
            m = { -m };
        }
        if (n < 0_i64) && (n != ({ std_numeric_limits_long_::min() })) {
            n = { -n };
        }
    }
    return (({ basic_gcd__110((m as u64), (n as u64)) }) as i64);
}
pub fn basic_gcd_115(mut m: u8, mut n: u8) -> u8 {
    {}
    return ({ basic_gcd__107((m as u8), (n as u8)) });
}
pub fn basic_gcd_116(mut m: u16, mut n: u16) -> u16 {
    {}
    return ({ basic_gcd__108((m as u16), (n as u16)) });
}
pub fn basic_gcd_117(mut m: u32, mut n: u32) -> u32 {
    {}
    return ({ basic_gcd__109((m as u32), (n as u32)) });
}
pub fn basic_gcd_118(mut m: u64, mut n: u64) -> u64 {
    {}
    return ({ basic_gcd__110((m as u64), (n as u64)) });
}
pub fn do_fuzzy_tests_119() {
    let gen_ : Value<std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_ > = Rc::new(RefCell::new(std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_ :: new_1 ( { 1938_u64  } , )  )) ;
    let mut max_input: i8 = 127_i8;
    let distrib: Value<std_uniform_int_distribution_int_> = Rc::new(RefCell::new(
        std_uniform_int_distribution_int_::new_1({ 0 }, { (max_input as i32) }),
    ));
    let mut nb_rounds: i32 = 10000;
    let mut i: i32 = 0;
    'loop_: while (i < nb_rounds) {
        let mut n: i8 = (({ (*distrib.borrow_mut())(gen_.as_pointer()) }) as i8);
        let mut m: i8 = (({ (*distrib.borrow_mut())(gen_.as_pointer()) }) as i8);
        ({
            cpp2rust_assert_fail_3(
                ((({ gcd_4(n, m) }) as i32) == (({ basic_gcd_111(n, m) }) as i32)),
            )
        });
        i.prefix_inc();
    }
}
pub fn do_fuzzy_tests_120() {
    let gen_ : Value<std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_ > = Rc::new(RefCell::new(std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_ :: new_1 ( { 1938_u64  } , )  )) ;
    let mut max_input: i16 = 32767_i16;
    let distrib: Value<std_uniform_int_distribution_short_> = Rc::new(RefCell::new(
        std_uniform_int_distribution_short_::new_1({ 0_i16 }, { max_input }),
    ));
    let mut nb_rounds: i32 = 10000;
    let mut i: i32 = 0;
    'loop_: while (i < nb_rounds) {
        let mut n: i16 = (({ (*distrib.borrow_mut())(gen_.as_pointer()) }) as i16);
        let mut m: i16 = (({ (*distrib.borrow_mut())(gen_.as_pointer()) }) as i16);
        ({
            cpp2rust_assert_fail_3(
                ((({ gcd_12(n, m) }) as i32) == (({ basic_gcd_112(n, m) }) as i32)),
            )
        });
        i.prefix_inc();
    }
}
pub fn do_fuzzy_tests_121() {
    let gen_ : Value<std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_ > = Rc::new(RefCell::new(std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_ :: new_1 ( { 1938_u64  } , )  )) ;
    let mut max_input: i32 = 2147483647;
    let distrib: Value<std_uniform_int_distribution_int_> = Rc::new(RefCell::new(
        std_uniform_int_distribution_int_::new_1({ 0 }, { max_input }),
    ));
    let mut nb_rounds: i32 = 10000;
    let mut i: i32 = 0;
    'loop_: while (i < nb_rounds) {
        let mut n: i32 = (({ (*distrib.borrow_mut())(gen_.as_pointer()) }) as i32);
        let mut m: i32 = (({ (*distrib.borrow_mut())(gen_.as_pointer()) }) as i32);
        ({ cpp2rust_assert_fail_3((({ gcd_20(n, m) }) == ({ basic_gcd_113(n, m) }))) });
        i.prefix_inc();
    }
}
pub fn do_fuzzy_tests_122() {
    let gen_ : Value<std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_ > = Rc::new(RefCell::new(std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_ :: new_1 ( { 1938_u64  } , )  )) ;
    let mut max_input: i64 = 9223372036854775807_i64;
    let distrib: Value<std_uniform_int_distribution_long_> = Rc::new(RefCell::new(
        std_uniform_int_distribution_long_::new_1({ 0_i64 }, { max_input }),
    ));
    let mut nb_rounds: i32 = 10000;
    let mut i: i32 = 0;
    'loop_: while (i < nb_rounds) {
        let mut n: i64 = (({ (*distrib.borrow_mut())(gen_.as_pointer()) }) as i64);
        let mut m: i64 = (({ (*distrib.borrow_mut())(gen_.as_pointer()) }) as i64);
        ({ cpp2rust_assert_fail_3((({ gcd_28(n, m) }) == ({ basic_gcd_114(n, m) }))) });
        i.prefix_inc();
    }
}
pub fn do_fuzzy_tests_123() {
    let gen_ : Value<std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_ > = Rc::new(RefCell::new(std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_ :: new_1 ( { 1938_u64  } , )  )) ;
    let mut max_input: u8 = 255_u8;
    let distrib: Value<std_uniform_int_distribution_int_> = Rc::new(RefCell::new(
        std_uniform_int_distribution_int_::new_1({ 0 }, { (max_input as i32) }),
    ));
    let mut nb_rounds: i32 = 10000;
    let mut i: i32 = 0;
    'loop_: while (i < nb_rounds) {
        let mut n: u8 = (({ (*distrib.borrow_mut())(gen_.as_pointer()) }) as u8);
        let mut m: u8 = (({ (*distrib.borrow_mut())(gen_.as_pointer()) }) as u8);
        ({
            cpp2rust_assert_fail_3(
                ((({ gcd_6(n, m) }) as i32) == (({ basic_gcd_115(n, m) }) as i32)),
            )
        });
        i.prefix_inc();
    }
}
pub fn do_fuzzy_tests_124() {
    let gen_ : Value<std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_ > = Rc::new(RefCell::new(std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_ :: new_1 ( { 1938_u64  } , )  )) ;
    let mut max_input: u16 = 65535_u16;
    let distrib: Value<std_uniform_int_distribution_unsigned_short_> = Rc::new(RefCell::new(
        std_uniform_int_distribution_unsigned_short_::new_1({ 0_u16 }, { max_input }),
    ));
    let mut nb_rounds: i32 = 10000;
    let mut i: i32 = 0;
    'loop_: while (i < nb_rounds) {
        let mut n: u16 = (({ (*distrib.borrow_mut())(gen_.as_pointer()) }) as u16);
        let mut m: u16 = (({ (*distrib.borrow_mut())(gen_.as_pointer()) }) as u16);
        ({
            cpp2rust_assert_fail_3(
                ((({ gcd_14(n, m) }) as i32) == (({ basic_gcd_116(n, m) }) as i32)),
            )
        });
        i.prefix_inc();
    }
}
pub fn do_fuzzy_tests_125() {
    let gen_ : Value<std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_ > = Rc::new(RefCell::new(std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_ :: new_1 ( { 1938_u64  } , )  )) ;
    let mut max_input: u32 = 4294967295_u32;
    let distrib: Value<std_uniform_int_distribution_unsigned_int_> = Rc::new(RefCell::new(
        std_uniform_int_distribution_unsigned_int_::new_1({ 0_u32 }, { max_input }),
    ));
    let mut nb_rounds: i32 = 10000;
    let mut i: i32 = 0;
    'loop_: while (i < nb_rounds) {
        let mut n: u32 = (({ (*distrib.borrow_mut())(gen_.as_pointer()) }) as u32);
        let mut m: u32 = (({ (*distrib.borrow_mut())(gen_.as_pointer()) }) as u32);
        ({ cpp2rust_assert_fail_3((({ gcd_22(n, m) }) == ({ basic_gcd_117(n, m) }))) });
        i.prefix_inc();
    }
}
pub fn do_fuzzy_tests_126() {
    let gen_ : Value<std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_ > = Rc::new(RefCell::new(std_mersenne_twister_engine_unsigned_long__32__624__397__31__2567483615__11__4294967295__7__2636928640__15__4022730752__18__1812433253_ :: new_1 ( { 1938_u64  } , )  )) ;
    let mut max_input: u64 = 18446744073709551615_u64;
    let distrib: Value<std_uniform_int_distribution_unsigned_long_> = Rc::new(RefCell::new(
        std_uniform_int_distribution_unsigned_long_::new_1({ 0_u64 }, { max_input }),
    ));
    let mut nb_rounds: i32 = 10000;
    let mut i: i32 = 0;
    'loop_: while (i < nb_rounds) {
        let mut n: u64 = (({ (*distrib.borrow_mut())(gen_.as_pointer()) }) as u64);
        let mut m: u64 = (({ (*distrib.borrow_mut())(gen_.as_pointer()) }) as u64);
        ({ cpp2rust_assert_fail_3((({ gcd_30(n, m) }) == ({ basic_gcd_118(n, m) }))) });
        i.prefix_inc();
    }
}
pub fn do_limit_tests_127() {
    let inputs: Value<Box<[i8]>> = Rc::new(RefCell::new(Box::new([
        (((({ std_numeric_limits_signed_char_::min() }) as i32)
            + (if value_128.with(|rc| *rc.borrow()) {
                3
            } else {
                0
            })) as i8),
        (((({ std_numeric_limits_signed_char_::min() }) as i32) + 1) as i8),
        (((({ std_numeric_limits_signed_char_::min() }) as i32) + 2) as i8),
        ({ std_numeric_limits_signed_char_::max() }),
        (((({ std_numeric_limits_signed_char_::max() }) as i32) - 1) as i8),
        (((({ std_numeric_limits_signed_char_::max() }) as i32) - 2) as i8),
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
    ])));
    'loop_: for n in IterRange::new(
        (inputs.as_pointer() as Ptr<i8>),
        (inputs.as_pointer() as Ptr<i8>).offset((27_i64) as isize),
    ) {
        let mut n: i8 = (n.read());
        'loop_: for m in IterRange::new(
            (inputs.as_pointer() as Ptr<i8>),
            (inputs.as_pointer() as Ptr<i8>).offset((27_i64) as isize),
        ) {
            let mut m: i8 = (m.read());
            ({
                cpp2rust_assert_fail_3(
                    ((({ gcd_4(n, m) }) as i32) == (({ basic_gcd_111(n, m) }) as i32)),
                )
            });
        }
    }
}
pub fn do_limit_tests_129() {
    let inputs: Value<Box<[i16]>> = Rc::new(RefCell::new(Box::new([
        (((({ std_numeric_limits_short_::min() }) as i32)
            + (if value_128.with(|rc| *rc.borrow()) {
                3
            } else {
                0
            })) as i16),
        (((({ std_numeric_limits_short_::min() }) as i32) + 1) as i16),
        (((({ std_numeric_limits_short_::min() }) as i32) + 2) as i16),
        ({ std_numeric_limits_short_::max() }),
        (((({ std_numeric_limits_short_::max() }) as i32) - 1) as i16),
        (((({ std_numeric_limits_short_::max() }) as i32) - 2) as i16),
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
    ])));
    'loop_: for n in IterRange::new(
        (inputs.as_pointer() as Ptr<i16>),
        (inputs.as_pointer() as Ptr<i16>).offset((27_i64) as isize),
    ) {
        let mut n: i16 = (n.read());
        'loop_: for m in IterRange::new(
            (inputs.as_pointer() as Ptr<i16>),
            (inputs.as_pointer() as Ptr<i16>).offset((27_i64) as isize),
        ) {
            let mut m: i16 = (m.read());
            ({
                cpp2rust_assert_fail_3(
                    ((({ gcd_12(n, m) }) as i32) == (({ basic_gcd_112(n, m) }) as i32)),
                )
            });
        }
    }
}
pub fn do_limit_tests_130() {
    let inputs: Value<Box<[i32]>> = Rc::new(RefCell::new(Box::new([
        (({ std_numeric_limits_int_::min() })
            + (if value_128.with(|rc| *rc.borrow()) {
                3
            } else {
                0
            })),
        (({ std_numeric_limits_int_::min() }) + 1),
        (({ std_numeric_limits_int_::min() }) + 2),
        ({ std_numeric_limits_int_::max() }),
        (({ std_numeric_limits_int_::max() }) - 1),
        (({ std_numeric_limits_int_::max() }) - 2),
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
    ])));
    'loop_: for n in IterRange::new(
        (inputs.as_pointer() as Ptr<i32>),
        (inputs.as_pointer() as Ptr<i32>).offset((27_i64) as isize),
    ) {
        let mut n: i32 = (n.read());
        'loop_: for m in IterRange::new(
            (inputs.as_pointer() as Ptr<i32>),
            (inputs.as_pointer() as Ptr<i32>).offset((27_i64) as isize),
        ) {
            let mut m: i32 = (m.read());
            ({ cpp2rust_assert_fail_3((({ gcd_20(n, m) }) == ({ basic_gcd_113(n, m) }))) });
        }
    }
}
pub fn do_limit_tests_131() {
    let inputs: Value<Box<[i64]>> = Rc::new(RefCell::new(Box::new([
        (({ std_numeric_limits_long_::min() })
            + ((if value_128.with(|rc| *rc.borrow()) {
                3
            } else {
                0
            }) as i64)),
        (({ std_numeric_limits_long_::min() }) + 1_i64),
        (({ std_numeric_limits_long_::min() }) + 2_i64),
        ({ std_numeric_limits_long_::max() }),
        (({ std_numeric_limits_long_::max() }) - 1_i64),
        (({ std_numeric_limits_long_::max() }) - 2_i64),
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
    ])));
    'loop_: for n in IterRange::new(
        (inputs.as_pointer() as Ptr<i64>),
        (inputs.as_pointer() as Ptr<i64>).offset((27_i64) as isize),
    ) {
        let mut n: i64 = (n.read());
        'loop_: for m in IterRange::new(
            (inputs.as_pointer() as Ptr<i64>),
            (inputs.as_pointer() as Ptr<i64>).offset((27_i64) as isize),
        ) {
            let mut m: i64 = (m.read());
            ({ cpp2rust_assert_fail_3((({ gcd_28(n, m) }) == ({ basic_gcd_114(n, m) }))) });
        }
    }
}
pub fn do_limit_tests_132() {
    let inputs: Value<Box<[u8]>> = Rc::new(RefCell::new(Box::new([
        (((({ std_numeric_limits_unsigned_char_::min() }) as i32)
            + (if value_133.with(|rc| *rc.borrow()) {
                3
            } else {
                0
            })) as u8),
        (((({ std_numeric_limits_unsigned_char_::min() }) as i32) + 1) as u8),
        (((({ std_numeric_limits_unsigned_char_::min() }) as i32) + 2) as u8),
        ({ std_numeric_limits_unsigned_char_::max() }),
        (((({ std_numeric_limits_unsigned_char_::max() }) as i32) - 1) as u8),
        (((({ std_numeric_limits_unsigned_char_::max() }) as i32) - 2) as u8),
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
    ])));
    'loop_: for n in IterRange::new(
        (inputs.as_pointer() as Ptr<u8>),
        (inputs.as_pointer() as Ptr<u8>).offset((27_i64) as isize),
    ) {
        let mut n: u8 = (n.read());
        'loop_: for m in IterRange::new(
            (inputs.as_pointer() as Ptr<u8>),
            (inputs.as_pointer() as Ptr<u8>).offset((27_i64) as isize),
        ) {
            let mut m: u8 = (m.read());
            ({
                cpp2rust_assert_fail_3(
                    ((({ gcd_6(n, m) }) as i32) == (({ basic_gcd_115(n, m) }) as i32)),
                )
            });
        }
    }
}
pub fn do_limit_tests_134() {
    let inputs: Value<Box<[u16]>> = Rc::new(RefCell::new(Box::new([
        (((({ std_numeric_limits_unsigned_short_::min() }) as i32)
            + (if value_133.with(|rc| *rc.borrow()) {
                3
            } else {
                0
            })) as u16),
        (((({ std_numeric_limits_unsigned_short_::min() }) as i32) + 1) as u16),
        (((({ std_numeric_limits_unsigned_short_::min() }) as i32) + 2) as u16),
        ({ std_numeric_limits_unsigned_short_::max() }),
        (((({ std_numeric_limits_unsigned_short_::max() }) as i32) - 1) as u16),
        (((({ std_numeric_limits_unsigned_short_::max() }) as i32) - 2) as u16),
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
    ])));
    'loop_: for n in IterRange::new(
        (inputs.as_pointer() as Ptr<u16>),
        (inputs.as_pointer() as Ptr<u16>).offset((27_i64) as isize),
    ) {
        let mut n: u16 = (n.read());
        'loop_: for m in IterRange::new(
            (inputs.as_pointer() as Ptr<u16>),
            (inputs.as_pointer() as Ptr<u16>).offset((27_i64) as isize),
        ) {
            let mut m: u16 = (m.read());
            ({
                cpp2rust_assert_fail_3(
                    ((({ gcd_14(n, m) }) as i32) == (({ basic_gcd_116(n, m) }) as i32)),
                )
            });
        }
    }
}
pub fn do_limit_tests_135() {
    let inputs: Value<Box<[u32]>> = Rc::new(RefCell::new(Box::new([
        (({ std_numeric_limits_unsigned_int_::min() }) as u32).wrapping_add(
            ((if value_133.with(|rc| *rc.borrow()) {
                3
            } else {
                0
            }) as u32),
        ),
        (({ std_numeric_limits_unsigned_int_::min() }) as u32).wrapping_add(1_u32),
        (({ std_numeric_limits_unsigned_int_::min() }) as u32).wrapping_add(2_u32),
        ({ std_numeric_limits_unsigned_int_::max() }),
        (({ std_numeric_limits_unsigned_int_::max() }) as u32).wrapping_sub(1_u32),
        (({ std_numeric_limits_unsigned_int_::max() }) as u32).wrapping_sub(2_u32),
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
    ])));
    'loop_: for n in IterRange::new(
        (inputs.as_pointer() as Ptr<u32>),
        (inputs.as_pointer() as Ptr<u32>).offset((27_i64) as isize),
    ) {
        let mut n: u32 = (n.read());
        'loop_: for m in IterRange::new(
            (inputs.as_pointer() as Ptr<u32>),
            (inputs.as_pointer() as Ptr<u32>).offset((27_i64) as isize),
        ) {
            let mut m: u32 = (m.read());
            ({ cpp2rust_assert_fail_3((({ gcd_22(n, m) }) == ({ basic_gcd_117(n, m) }))) });
        }
    }
}
pub fn do_limit_tests_136() {
    let inputs: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        (({ std_numeric_limits_unsigned_long_::min() }) as u64).wrapping_add(
            ((if value_133.with(|rc| *rc.borrow()) {
                3
            } else {
                0
            }) as u64),
        ),
        (({ std_numeric_limits_unsigned_long_::min() }) as u64).wrapping_add(1_u64),
        (({ std_numeric_limits_unsigned_long_::min() }) as u64).wrapping_add(2_u64),
        ({ std_numeric_limits_unsigned_long_::max() }),
        (({ std_numeric_limits_unsigned_long_::max() }) as u64).wrapping_sub(1_u64),
        (({ std_numeric_limits_unsigned_long_::max() }) as u64).wrapping_sub(2_u64),
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
    ])));
    'loop_: for n in IterRange::new(
        (inputs.as_pointer() as Ptr<u64>),
        (inputs.as_pointer() as Ptr<u64>).offset((27_i64) as isize),
    ) {
        let mut n: u64 = (n.read());
        'loop_: for m in IterRange::new(
            (inputs.as_pointer() as Ptr<u64>),
            (inputs.as_pointer() as Ptr<u64>).offset((27_i64) as isize),
        ) {
            let mut m: u64 = (m.read());
            ({ cpp2rust_assert_fail_3((({ gcd_30(n, m) }) == ({ basic_gcd_118(n, m) }))) });
        }
    }
}
pub fn do_test_137(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((12_i64) as isize),
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
                        test0_5(_in1, _in2, _out)
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
                        test0_7(_in1, _in2, _out)
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
                        test0_7(_in1, _in2, _out)
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
pub fn do_test_138(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
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
                        let _in1: i32 = -TC.x;
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
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_11(_in1, _in2, _out)
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
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
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
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_11(_in1, _in2, _out)
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
                        test0_13(_in1, _in2, _out)
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
                        test0_15(_in1, _in2, _out)
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
                        test0_15(_in1, _in2, _out)
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
pub fn do_test_139(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
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
                        let _in1: i32 = -TC.x;
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
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_19(_in1, _in2, _out)
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
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
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
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_19(_in1, _in2, _out)
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
                        test0_21(_in1, _in2, _out)
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
        }
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
                        test0_25(_in1, _in2, _out)
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
                        test0_25(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_140(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
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
                        test0_27(_in1, _in2, _out)
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
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_27(_in1, _in2, _out)
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
                        test0_27(_in1, _in2, _out)
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
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
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
                        test0_29(_in1, _in2, _out)
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
        }
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
                        test0_33(_in1, _in2, _out)
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
                        test0_33(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_141(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
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
                        test0_35(_in1, _in2, _out)
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
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_35(_in1, _in2, _out)
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
                        test0_35(_in1, _in2, _out)
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
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
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
                        test0_37(_in1, _in2, _out)
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
        }
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
                        test0_41(_in1, _in2, _out)
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
                        test0_41(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_142(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
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
                        test0_43(_in1, _in2, _out)
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
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_43(_in1, _in2, _out)
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
                        test0_45(_in1, _in2, _out)
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
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
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
                        test0_51(_in1, _in2, _out)
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
                        test0_51(_in1, _in2, _out)
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
                        test0_55(_in1, _in2, _out)
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
                        test0_57(_in1, _in2, _out)
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
                        test0_55(_in1, _in2, _out)
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
                        test0_57(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_143(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
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
                        test0_45(_in1, _in2, _out)
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
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
                        let _in2: i32 = -TC.y;
                        let _out: i32 = TC.expect;
                        test0_45(_in1, _in2, _out)
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
                        test0_43(_in1, _in2, _out)
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
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = -TC.x;
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
                        test0_55(_in1, _in2, _out)
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
                        test0_57(_in1, _in2, _out)
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
                        test0_55(_in1, _in2, _out)
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
                        test0_57(_in1, _in2, _out)
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
                        test0_51(_in1, _in2, _out)
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
                        test0_51(_in1, _in2, _out)
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
pub fn do_test_144(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_59(_in1, _in2, _out)
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
                        test0_59(_in1, _in2, _out)
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
                        test0_59(_in1, _in2, _out)
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
                        test0_59(_in1, _in2, _out)
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
                        test0_61(_in1, _in2, _out)
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
                        test0_61(_in1, _in2, _out)
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
                        test0_61(_in1, _in2, _out)
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
                        test0_61(_in1, _in2, _out)
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
                        test0_63(_in1, _in2, _out)
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
                        test0_65(_in1, _in2, _out)
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
                        test0_67(_in1, _in2, _out)
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
                        test0_69(_in1, _in2, _out)
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
                        test0_67(_in1, _in2, _out)
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
                        test0_69(_in1, _in2, _out)
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
                        test0_71(_in1, _in2, _out)
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
                        test0_73(_in1, _in2, _out)
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
                        test0_71(_in1, _in2, _out)
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
                        test0_73(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_145(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_61(_in1, _in2, _out)
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
                        test0_61(_in1, _in2, _out)
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
                        test0_61(_in1, _in2, _out)
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
                        test0_61(_in1, _in2, _out)
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
                        test0_59(_in1, _in2, _out)
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
                        test0_59(_in1, _in2, _out)
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
                        test0_59(_in1, _in2, _out)
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
                        test0_59(_in1, _in2, _out)
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
                        test0_65(_in1, _in2, _out)
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
                        test0_63(_in1, _in2, _out)
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
                        test0_71(_in1, _in2, _out)
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
                        test0_73(_in1, _in2, _out)
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
                        test0_71(_in1, _in2, _out)
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
                        test0_73(_in1, _in2, _out)
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
                        test0_67(_in1, _in2, _out)
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
                        test0_69(_in1, _in2, _out)
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
                        test0_67(_in1, _in2, _out)
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
                        test0_69(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_146(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_75(_in1, _in2, _out)
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
                        test0_75(_in1, _in2, _out)
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
                        test0_75(_in1, _in2, _out)
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
                        test0_75(_in1, _in2, _out)
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
                        test0_77(_in1, _in2, _out)
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
                        test0_77(_in1, _in2, _out)
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
                        test0_77(_in1, _in2, _out)
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
                        test0_77(_in1, _in2, _out)
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
                        test0_79(_in1, _in2, _out)
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
                        test0_81(_in1, _in2, _out)
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
                        test0_83(_in1, _in2, _out)
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
                        test0_85(_in1, _in2, _out)
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
                        test0_83(_in1, _in2, _out)
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
                        test0_85(_in1, _in2, _out)
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
                        test0_87(_in1, _in2, _out)
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
                        test0_89(_in1, _in2, _out)
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
                        test0_87(_in1, _in2, _out)
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
                        test0_89(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_147(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_77(_in1, _in2, _out)
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
                        test0_77(_in1, _in2, _out)
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
                        test0_77(_in1, _in2, _out)
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
                        test0_77(_in1, _in2, _out)
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
                        test0_75(_in1, _in2, _out)
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
                        test0_75(_in1, _in2, _out)
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
                        test0_75(_in1, _in2, _out)
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
                        test0_75(_in1, _in2, _out)
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
                        test0_81(_in1, _in2, _out)
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
                        test0_79(_in1, _in2, _out)
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
                        test0_87(_in1, _in2, _out)
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
                        test0_89(_in1, _in2, _out)
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
                        test0_87(_in1, _in2, _out)
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
                        test0_89(_in1, _in2, _out)
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
                        test0_83(_in1, _in2, _out)
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
                        test0_85(_in1, _in2, _out)
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
                        test0_83(_in1, _in2, _out)
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
                        test0_85(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_148(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_91(_in1, _in2, _out)
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
                        test0_91(_in1, _in2, _out)
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
                        test0_91(_in1, _in2, _out)
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
                        test0_91(_in1, _in2, _out)
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
                        test0_93(_in1, _in2, _out)
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
                        test0_93(_in1, _in2, _out)
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
                        test0_93(_in1, _in2, _out)
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
                        test0_93(_in1, _in2, _out)
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
                        test0_95(_in1, _in2, _out)
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
                        test0_97(_in1, _in2, _out)
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
                        test0_99(_in1, _in2, _out)
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
                        test0_101(_in1, _in2, _out)
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
                        test0_99(_in1, _in2, _out)
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
                        test0_101(_in1, _in2, _out)
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
                        test0_103(_in1, _in2, _out)
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
                        test0_105(_in1, _in2, _out)
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
                        test0_103(_in1, _in2, _out)
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
                        test0_105(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn do_test_149(_a0: Option<i32>) -> bool {
    let mut _a0: i32 = _a0.unwrap_or_else(|| 0);
    let accumulate: Value<bool> = Rc::new(RefCell::new(true));
    'loop_: for TC in IterRange::new(
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>),
        (Cases_1.with(|v| v.as_pointer()) as Ptr<anon_0>).offset((12_i64) as isize),
    ) {
        let mut TC: anon_0 = (*TC.upgrade().deref()).clone();
        {
            {
                let rhs_0 = (((*accumulate.borrow()) as i32)
                    & (({
                        let _in1: i32 = TC.x;
                        let _in2: i32 = TC.y;
                        let _out: i32 = TC.expect;
                        test0_93(_in1, _in2, _out)
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
                        test0_93(_in1, _in2, _out)
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
                        test0_93(_in1, _in2, _out)
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
                        test0_93(_in1, _in2, _out)
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
                        test0_91(_in1, _in2, _out)
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
                        test0_91(_in1, _in2, _out)
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
                        test0_91(_in1, _in2, _out)
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
                        test0_91(_in1, _in2, _out)
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
                        test0_97(_in1, _in2, _out)
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
                        test0_95(_in1, _in2, _out)
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
                        test0_103(_in1, _in2, _out)
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
                        test0_105(_in1, _in2, _out)
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
                        test0_103(_in1, _in2, _out)
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
                        test0_105(_in1, _in2, _out)
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
                        test0_99(_in1, _in2, _out)
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
                        test0_101(_in1, _in2, _out)
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
                        test0_99(_in1, _in2, _out)
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
                        test0_101(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
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
    ({ cpp2rust_assert_fail_3(({ do_test_137(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_138(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_139(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_140(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_141(Some(non_cce)) })) });
    const _: () = assert!(true, "do_test<std::int8_t>()");;
    const _: () = assert!(true, "do_test<std::int16_t>()");;
    const _: () = assert!(true, "do_test<std::int32_t>()");;
    const _: () = assert!(true, "do_test<std::int64_t>()");;
    ({ cpp2rust_assert_fail_3(({ do_test_137(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_138(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_139(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_140(Some(non_cce)) })) });
    const _: () = assert!(true, "do_test<signed char, int>()");;
    const _: () = assert!(true, "do_test<int, signed char>()");;
    const _: () = assert!(true, "do_test<short, int>()");;
    const _: () = assert!(true, "do_test<int, short>()");;
    const _: () = assert!(true, "do_test<int, long>()");;
    const _: () = assert!(true, "do_test<long, int>()");;
    const _: () = assert!(true, "do_test<int, long long>()");;
    const _: () = assert!(true, "do_test<long long, int>()");;
    ({ cpp2rust_assert_fail_3(({ do_test_142(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_143(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_144(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_145(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_146(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_147(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_148(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_149(Some(non_cce)) })) });
    {
        let res: Value<i64> = Rc::new(RefCell::new(({ gcd_78(1234_i64, (-2147483647_i32 - 1)) })));
        const _: () = assert!(true, "std::is_same_v<decltype(res), std::int64_t>");;
        ({ cpp2rust_assert_fail_3(((*res.borrow()) == 2_i64)) });
    }
    ({ do_fuzzy_tests_119() });
    ({ do_fuzzy_tests_120() });
    ({ do_fuzzy_tests_121() });
    ({ do_fuzzy_tests_122() });
    ({ do_fuzzy_tests_123() });
    ({ do_fuzzy_tests_124() });
    ({ do_fuzzy_tests_125() });
    ({ do_fuzzy_tests_126() });
    ({ do_limit_tests_127() });
    ({ do_limit_tests_129() });
    ({ do_limit_tests_130() });
    ({ do_limit_tests_131() });
    ({ do_limit_tests_132() });
    ({ do_limit_tests_134() });
    ({ do_limit_tests_135() });
    ({ do_limit_tests_136() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = Cases_1.with(|_| ());
}
