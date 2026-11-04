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
    ({ cpp2rust_assert_fail_3((((out as i8) as i32) == (({ lcm_4(value1, value2) }) as i32))) });
    return true;
}
pub fn test0_5(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3((((out as u8) as i32) == (({ lcm_6(value1, value2) }) as i32))) });
    return true;
}
pub fn test0_7(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ lcm_8(value1, value2) }))) });
    return true;
}
pub fn test0_9(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ lcm_10(value1, value2) }))) });
    return true;
}
pub fn test0_11(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3((((out as i16) as i32) == (({ lcm_12(value1, value2) }) as i32))) });
    return true;
}
pub fn test0_13(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3((((out as u16) as i32) == (({ lcm_14(value1, value2) }) as i32))) });
    return true;
}
pub fn test0_15(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ lcm_16(value1, value2) }))) });
    return true;
}
pub fn test0_17(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ lcm_18(value1, value2) }))) });
    return true;
}
pub fn test0_19(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ lcm_20(value1, value2) }))) });
    return true;
}
pub fn test0_21(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ lcm_22(value1, value2) }))) });
    return true;
}
pub fn test0_23(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ lcm_24(value1, value2) }))) });
    return true;
}
pub fn test0_25(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ lcm_26(value1, value2) }))) });
    return true;
}
pub fn test0_27(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i64) == ({ lcm_28(value1, value2) }))) });
    return true;
}
pub fn test0_29(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ lcm_30(value1, value2) }))) });
    return true;
}
pub fn test0_31(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ lcm_32(value1, value2) }))) });
    return true;
}
pub fn test0_33(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ lcm_34(value1, value2) }))) });
    return true;
}
pub fn test0_35(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i64) == ({ lcm_36(value1, value2) }))) });
    return true;
}
pub fn test0_37(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ lcm_38(value1, value2) }))) });
    return true;
}
pub fn test0_39(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ lcm_40(value1, value2) }))) });
    return true;
}
pub fn test0_41(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ lcm_42(value1, value2) }))) });
    return true;
}
pub fn test0_43(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ lcm_44(value1, value2) }))) });
    return true;
}
pub fn test0_45(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ lcm_46(value1, value2) }))) });
    return true;
}
pub fn test0_47(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ lcm_48(value1, value2) }))) });
    return true;
}
pub fn test0_49(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ lcm_50(value1, value2) }))) });
    return true;
}
pub fn test0_51(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ lcm_52(value1, value2) }))) });
    return true;
}
pub fn test0_53(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ lcm_54(value1, value2) }))) });
    return true;
}
pub fn test0_55(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ lcm_56(value1, value2) }))) });
    return true;
}
pub fn test0_57(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ lcm_58(value1, value2) }))) });
    return true;
}
pub fn test0_59(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ lcm_60(value1, value2) }))) });
    return true;
}
pub fn test0_61(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ lcm_62(value1, value2) }))) });
    return true;
}
pub fn test0_63(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ lcm_64(value1, value2) }))) });
    return true;
}
pub fn test0_65(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ lcm_66(value1, value2) }))) });
    return true;
}
pub fn test0_67(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ lcm_68(value1, value2) }))) });
    return true;
}
pub fn test0_69(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u32) == ({ lcm_70(value1, value2) }))) });
    return true;
}
pub fn test0_71(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ lcm_72(value1, value2) }))) });
    return true;
}
pub fn test0_73(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i32) == ({ lcm_74(value1, value2) }))) });
    return true;
}
pub fn test0_75(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i64) == ({ lcm_76(value1, value2) }))) });
    return true;
}
pub fn test0_77(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i64) == ({ lcm_78(value1, value2) }))) });
    return true;
}
pub fn test0_79(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ lcm_80(value1, value2) }))) });
    return true;
}
pub fn test0_81(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ lcm_82(value1, value2) }))) });
    return true;
}
pub fn test0_83(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ lcm_84(value1, value2) }))) });
    return true;
}
pub fn test0_85(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ lcm_86(value1, value2) }))) });
    return true;
}
pub fn test0_87(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i64) == ({ lcm_88(value1, value2) }))) });
    return true;
}
pub fn test0_89(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i64) == ({ lcm_90(value1, value2) }))) });
    return true;
}
pub fn test0_91(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i64) == ({ lcm_92(value1, value2) }))) });
    return true;
}
pub fn test0_93(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i64) == ({ lcm_94(value1, value2) }))) });
    return true;
}
pub fn test0_95(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ lcm_96(value1, value2) }))) });
    return true;
}
pub fn test0_97(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ lcm_98(value1, value2) }))) });
    return true;
}
pub fn test0_99(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ lcm_100(value1, value2) }))) });
    return true;
}
pub fn test0_101(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as u64) == ({ lcm_102(value1, value2) }))) });
    return true;
}
pub fn test0_103(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i64) == ({ lcm_104(value1, value2) }))) });
    return true;
}
pub fn test0_105(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    ({ cpp2rust_assert_fail_3(((out as i64) == ({ lcm_106(value1, value2) }))) });
    return true;
}
pub fn do_test_107(_a0: Option<i32>) -> bool {
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
pub fn do_test_108(_a0: Option<i32>) -> bool {
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
pub fn do_test_109(_a0: Option<i32>) -> bool {
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
pub fn do_test_110(_a0: Option<i32>) -> bool {
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
pub fn do_test_111(_a0: Option<i32>) -> bool {
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
pub fn do_test_112(_a0: Option<i32>) -> bool {
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
pub fn do_test_113(_a0: Option<i32>) -> bool {
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
pub fn do_test_114(_a0: Option<i32>) -> bool {
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
pub fn do_test_115(_a0: Option<i32>) -> bool {
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
pub fn do_test_116(_a0: Option<i32>) -> bool {
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
pub fn do_test_117(_a0: Option<i32>) -> bool {
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
pub fn do_test_118(_a0: Option<i32>) -> bool {
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
pub fn do_test_119(_a0: Option<i32>) -> bool {
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
pub fn test_limits_120() -> bool {
    ({
        cpp2rust_assert_fail_3(
            (({
                let ___m: u32 =
                    (({ std_numeric_limits_unsigned_int_::max() }) as u32).wrapping_sub(1_u32);
                let ___n: u32 =
                    (({ std_numeric_limits_unsigned_int_::max() }) as u32).wrapping_sub(1_u32);
                lcm_22(___m, ___n)
            }) == (({ std_numeric_limits_unsigned_int_::max() }) as u32).wrapping_sub(1_u32)),
        )
    });
    ({
        cpp2rust_assert_fail_3(
            (({
                let ___m: u32 = ({ std_numeric_limits_unsigned_int_::max() });
                let ___n: u32 = ({ std_numeric_limits_unsigned_int_::max() });
                lcm_22(___m, ___n)
            }) == ({ std_numeric_limits_unsigned_int_::max() })),
        )
    });
    return true;
}
pub fn test_limits_121() -> bool {
    ({
        cpp2rust_assert_fail_3(
            (({
                let ___m: u64 =
                    (({ std_numeric_limits_unsigned_long_::max() }) as u64).wrapping_sub(1_u64);
                let ___n: u64 =
                    (({ std_numeric_limits_unsigned_long_::max() }) as u64).wrapping_sub(1_u64);
                lcm_30(___m, ___n)
            }) == (({ std_numeric_limits_unsigned_long_::max() }) as u64).wrapping_sub(1_u64)),
        )
    });
    ({
        cpp2rust_assert_fail_3(
            (({
                let ___m: u64 = ({ std_numeric_limits_unsigned_long_::max() });
                let ___n: u64 = ({ std_numeric_limits_unsigned_long_::max() });
                lcm_30(___m, ___n)
            }) == ({ std_numeric_limits_unsigned_long_::max() })),
        )
    });
    return true;
}
pub fn test_limits_122() -> bool {
    ({
        cpp2rust_assert_fail_3(
            (({
                let ___m: i32 = (({ std_numeric_limits_int_::max() }) - 1);
                let ___n: i32 = (({ std_numeric_limits_int_::max() }) - 1);
                lcm_20(___m, ___n)
            }) == (({ std_numeric_limits_int_::max() }) - 1)),
        )
    });
    ({
        cpp2rust_assert_fail_3(
            (({
                let ___m: i32 = ({ std_numeric_limits_int_::max() });
                let ___n: i32 = ({ std_numeric_limits_int_::max() });
                lcm_20(___m, ___n)
            }) == ({ std_numeric_limits_int_::max() })),
        )
    });
    return true;
}
pub fn test_limits_123() -> bool {
    ({
        cpp2rust_assert_fail_3(
            (({
                let ___m: i64 = (({ std_numeric_limits_long_::max() }) - 1_i64);
                let ___n: i64 = (({ std_numeric_limits_long_::max() }) - 1_i64);
                lcm_28(___m, ___n)
            }) == (({ std_numeric_limits_long_::max() }) - 1_i64)),
        )
    });
    ({
        cpp2rust_assert_fail_3(
            (({
                let ___m: i64 = ({ std_numeric_limits_long_::max() });
                let ___n: i64 = ({ std_numeric_limits_long_::max() });
                lcm_28(___m, ___n)
            }) == ({ std_numeric_limits_long_::max() })),
        )
    });
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
    ({ cpp2rust_assert_fail_3(({ do_test_107(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_108(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_109(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_110(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_111(Some(non_cce)) })) });
    const _: () = assert!(true, "do_test<std::int8_t>()");;
    const _: () = assert!(true, "do_test<std::int16_t>()");;
    const _: () = assert!(true, "do_test<std::int32_t>()");;
    const _: () = assert!(true, "do_test<std::int64_t>()");;
    ({ cpp2rust_assert_fail_3(({ do_test_107(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_108(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_109(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_110(Some(non_cce)) })) });
    const _: () = assert!(true, "do_test<signed char, int>()");;
    const _: () = assert!(true, "do_test<int, signed char>()");;
    const _: () = assert!(true, "do_test<short, int>()");;
    const _: () = assert!(true, "do_test<int, short>()");;
    const _: () = assert!(true, "do_test<int, long>()");;
    const _: () = assert!(true, "do_test<long, int>()");;
    const _: () = assert!(true, "do_test<int, long long>()");;
    const _: () = assert!(true, "do_test<long long, int>()");;
    ({ cpp2rust_assert_fail_3(({ do_test_112(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_113(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_114(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_115(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_116(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_117(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_118(Some(non_cce)) })) });
    ({ cpp2rust_assert_fail_3(({ do_test_119(Some(non_cce)) })) });
    {
        let res1: Value<i64> = Rc::new(RefCell::new(({ lcm_78(1234_i64, (-2147483647_i32 - 1)) })));
        &({ lcm_84((-2147483647_i32 - 1), 2_u64) });
        const _: () = assert!(true, "std::is_same_v<decltype(res1), std::int64_t>");;
        ({ cpp2rust_assert_fail_3(((*res1.borrow()) == 1324997410816_i64)) });
    }
    {
        ({ cpp2rust_assert_fail_3(({ test_limits_120() })) });
        ({ cpp2rust_assert_fail_3(({ test_limits_120() })) });
        ({ cpp2rust_assert_fail_3(({ test_limits_121() })) });
        ({ cpp2rust_assert_fail_3(({ test_limits_122() })) });
        ({ cpp2rust_assert_fail_3(({ test_limits_122() })) });
        ({ cpp2rust_assert_fail_3(({ test_limits_123() })) });
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
