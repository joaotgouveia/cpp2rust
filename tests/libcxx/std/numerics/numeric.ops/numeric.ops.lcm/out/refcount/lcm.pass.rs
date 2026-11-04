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
    assert!((((out as i8) as i32) == (({ lcm_3(value1, value2,) }) as i32)));
    return true;
}
pub fn test0_4(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!((((out as u8) as i32) == (({ lcm_5(value1, value2,) }) as i32)));
    return true;
}
pub fn test0_6(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i32) == ({ lcm_7(value1, value2,) })));
    return true;
}
pub fn test0_8(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i32) == ({ lcm_9(value1, value2,) })));
    return true;
}
pub fn test0_10(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!((((out as i16) as i32) == (({ lcm_11(value1, value2,) }) as i32)));
    return true;
}
pub fn test0_12(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!((((out as u16) as i32) == (({ lcm_13(value1, value2,) }) as i32)));
    return true;
}
pub fn test0_14(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i32) == ({ lcm_15(value1, value2,) })));
    return true;
}
pub fn test0_16(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i32) == ({ lcm_17(value1, value2,) })));
    return true;
}
pub fn test0_18(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i32) == ({ lcm_19(value1, value2,) })));
    return true;
}
pub fn test0_20(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u32) == ({ lcm_21(value1, value2,) })));
    return true;
}
pub fn test0_22(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u32) == ({ lcm_23(value1, value2,) })));
    return true;
}
pub fn test0_24(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u32) == ({ lcm_25(value1, value2,) })));
    return true;
}
pub fn test0_26(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i64) == ({ lcm_27(value1, value2,) })));
    return true;
}
pub fn test0_28(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u64) == ({ lcm_29(value1, value2,) })));
    return true;
}
pub fn test0_30(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u64) == ({ lcm_31(value1, value2,) })));
    return true;
}
pub fn test0_32(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u64) == ({ lcm_33(value1, value2,) })));
    return true;
}
pub fn test0_34(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i64) == ({ lcm_35(value1, value2,) })));
    return true;
}
pub fn test0_36(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u64) == ({ lcm_37(value1, value2,) })));
    return true;
}
pub fn test0_38(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u64) == ({ lcm_39(value1, value2,) })));
    return true;
}
pub fn test0_40(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u64) == ({ lcm_41(value1, value2,) })));
    return true;
}
pub fn test0_42(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i32) == ({ lcm_43(value1, value2,) })));
    return true;
}
pub fn test0_44(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i32) == ({ lcm_45(value1, value2,) })));
    return true;
}
pub fn test0_46(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u32) == ({ lcm_47(value1, value2,) })));
    return true;
}
pub fn test0_48(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u32) == ({ lcm_49(value1, value2,) })));
    return true;
}
pub fn test0_50(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u32) == ({ lcm_51(value1, value2,) })));
    return true;
}
pub fn test0_52(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u32) == ({ lcm_53(value1, value2,) })));
    return true;
}
pub fn test0_54(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i32) == ({ lcm_55(value1, value2,) })));
    return true;
}
pub fn test0_56(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i32) == ({ lcm_57(value1, value2,) })));
    return true;
}
pub fn test0_58(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i32) == ({ lcm_59(value1, value2,) })));
    return true;
}
pub fn test0_60(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i32) == ({ lcm_61(value1, value2,) })));
    return true;
}
pub fn test0_62(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u32) == ({ lcm_63(value1, value2,) })));
    return true;
}
pub fn test0_64(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u32) == ({ lcm_65(value1, value2,) })));
    return true;
}
pub fn test0_66(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u32) == ({ lcm_67(value1, value2,) })));
    return true;
}
pub fn test0_68(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u32) == ({ lcm_69(value1, value2,) })));
    return true;
}
pub fn test0_70(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i32) == ({ lcm_71(value1, value2,) })));
    return true;
}
pub fn test0_72(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i32) == ({ lcm_73(value1, value2,) })));
    return true;
}
pub fn test0_74(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i64) == ({ lcm_75(value1, value2,) })));
    return true;
}
pub fn test0_76(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i64) == ({ lcm_77(value1, value2,) })));
    return true;
}
pub fn test0_78(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u64) == ({ lcm_79(value1, value2,) })));
    return true;
}
pub fn test0_80(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u64) == ({ lcm_81(value1, value2,) })));
    return true;
}
pub fn test0_82(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u64) == ({ lcm_83(value1, value2,) })));
    return true;
}
pub fn test0_84(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u64) == ({ lcm_85(value1, value2,) })));
    return true;
}
pub fn test0_86(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i64) == ({ lcm_87(value1, value2,) })));
    return true;
}
pub fn test0_88(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i64) == ({ lcm_89(value1, value2,) })));
    return true;
}
pub fn test0_90(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i64) == ({ lcm_91(value1, value2,) })));
    return true;
}
pub fn test0_92(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i64) == ({ lcm_93(value1, value2,) })));
    return true;
}
pub fn test0_94(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u64) == ({ lcm_95(value1, value2,) })));
    return true;
}
pub fn test0_96(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u64) == ({ lcm_97(value1, value2,) })));
    return true;
}
pub fn test0_98(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u64) == ({ lcm_99(value1, value2,) })));
    return true;
}
pub fn test0_100(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as u64) == ({ lcm_101(value1, value2,) })));
    return true;
}
pub fn test0_102(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i64) == ({ lcm_103(value1, value2,) })));
    return true;
}
pub fn test0_104(mut in1: i32, mut in2: i32, mut out: i32) -> bool {
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
    assert!(((out as i64) == ({ lcm_105(value1, value2,) })));
    return true;
}
pub fn do_test_106(_a0: Option<i32>) -> bool {
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
                        test0_4(_in1, _in2, _out)
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
                        test0_8(_in1, _in2, _out)
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
                        test0_8(_in1, _in2, _out)
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
                        test0_8(_in1, _in2, _out)
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
                        test0_8(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
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
                        test0_12(_in1, _in2, _out)
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
                        test0_16(_in1, _in2, _out)
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
                        test0_16(_in1, _in2, _out)
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
                        test0_16(_in1, _in2, _out)
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
                        test0_16(_in1, _in2, _out)
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
                        test0_20(_in1, _in2, _out)
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
                        test0_24(_in1, _in2, _out)
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
                        test0_24(_in1, _in2, _out)
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
                        test0_24(_in1, _in2, _out)
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
                        test0_26(_in1, _in2, _out)
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
                        test0_26(_in1, _in2, _out)
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
                        test0_26(_in1, _in2, _out)
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
                        test0_26(_in1, _in2, _out)
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
                        test0_26(_in1, _in2, _out)
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
                        test0_28(_in1, _in2, _out)
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
                        test0_32(_in1, _in2, _out)
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
                        test0_32(_in1, _in2, _out)
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
                        test0_32(_in1, _in2, _out)
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
                        test0_34(_in1, _in2, _out)
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
                        test0_34(_in1, _in2, _out)
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
                        test0_34(_in1, _in2, _out)
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
                        test0_34(_in1, _in2, _out)
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
                        test0_34(_in1, _in2, _out)
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
                        test0_36(_in1, _in2, _out)
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
                        test0_40(_in1, _in2, _out)
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
                        test0_40(_in1, _in2, _out)
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
                        test0_40(_in1, _in2, _out)
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
                        test0_42(_in1, _in2, _out)
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
                        test0_42(_in1, _in2, _out)
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
                        test0_44(_in1, _in2, _out)
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
                        test0_44(_in1, _in2, _out)
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
                        test0_44(_in1, _in2, _out)
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
                        test0_52(_in1, _in2, _out)
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
                        test0_52(_in1, _in2, _out)
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
                        test0_54(_in1, _in2, _out)
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
                        test0_56(_in1, _in2, _out)
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
                        test0_54(_in1, _in2, _out)
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
                        test0_56(_in1, _in2, _out)
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
                        test0_44(_in1, _in2, _out)
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
                        test0_44(_in1, _in2, _out)
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
                        test0_42(_in1, _in2, _out)
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
                        test0_42(_in1, _in2, _out)
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
                        test0_42(_in1, _in2, _out)
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
                        test0_54(_in1, _in2, _out)
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
                        test0_56(_in1, _in2, _out)
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
                        test0_54(_in1, _in2, _out)
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
                        test0_56(_in1, _in2, _out)
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
                        test0_52(_in1, _in2, _out)
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
                        test0_52(_in1, _in2, _out)
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
                        test0_58(_in1, _in2, _out)
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
                        test0_58(_in1, _in2, _out)
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
                        test0_58(_in1, _in2, _out)
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
                        test0_58(_in1, _in2, _out)
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
                        test0_60(_in1, _in2, _out)
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
                        test0_60(_in1, _in2, _out)
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
                        test0_60(_in1, _in2, _out)
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
                        test0_60(_in1, _in2, _out)
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
                        test0_62(_in1, _in2, _out)
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
                        test0_64(_in1, _in2, _out)
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
                        test0_66(_in1, _in2, _out)
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
                        test0_68(_in1, _in2, _out)
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
                        test0_66(_in1, _in2, _out)
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
                        test0_68(_in1, _in2, _out)
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
                        test0_70(_in1, _in2, _out)
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
                        test0_72(_in1, _in2, _out)
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
                        test0_70(_in1, _in2, _out)
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
                        test0_72(_in1, _in2, _out)
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
                        test0_60(_in1, _in2, _out)
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
                        test0_60(_in1, _in2, _out)
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
                        test0_60(_in1, _in2, _out)
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
                        test0_60(_in1, _in2, _out)
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
                        test0_58(_in1, _in2, _out)
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
                        test0_58(_in1, _in2, _out)
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
                        test0_58(_in1, _in2, _out)
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
                        test0_58(_in1, _in2, _out)
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
                        test0_64(_in1, _in2, _out)
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
                        test0_62(_in1, _in2, _out)
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
                        test0_70(_in1, _in2, _out)
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
                        test0_72(_in1, _in2, _out)
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
                        test0_70(_in1, _in2, _out)
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
                        test0_72(_in1, _in2, _out)
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
                        test0_66(_in1, _in2, _out)
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
                        test0_68(_in1, _in2, _out)
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
                        test0_66(_in1, _in2, _out)
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
                        test0_68(_in1, _in2, _out)
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
                        test0_74(_in1, _in2, _out)
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
                        test0_74(_in1, _in2, _out)
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
                        test0_74(_in1, _in2, _out)
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
                        test0_74(_in1, _in2, _out)
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
                        test0_76(_in1, _in2, _out)
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
                        test0_76(_in1, _in2, _out)
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
                        test0_76(_in1, _in2, _out)
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
                        test0_76(_in1, _in2, _out)
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
                        test0_78(_in1, _in2, _out)
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
                        test0_80(_in1, _in2, _out)
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
                        test0_82(_in1, _in2, _out)
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
                        test0_84(_in1, _in2, _out)
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
                        test0_82(_in1, _in2, _out)
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
                        test0_84(_in1, _in2, _out)
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
                        test0_86(_in1, _in2, _out)
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
                        test0_88(_in1, _in2, _out)
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
                        test0_86(_in1, _in2, _out)
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
                        test0_88(_in1, _in2, _out)
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
                        test0_76(_in1, _in2, _out)
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
                        test0_76(_in1, _in2, _out)
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
                        test0_76(_in1, _in2, _out)
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
                        test0_76(_in1, _in2, _out)
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
                        test0_74(_in1, _in2, _out)
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
                        test0_74(_in1, _in2, _out)
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
                        test0_74(_in1, _in2, _out)
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
                        test0_74(_in1, _in2, _out)
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
                        test0_80(_in1, _in2, _out)
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
                        test0_78(_in1, _in2, _out)
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
                        test0_86(_in1, _in2, _out)
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
                        test0_88(_in1, _in2, _out)
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
                        test0_86(_in1, _in2, _out)
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
                        test0_88(_in1, _in2, _out)
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
                        test0_82(_in1, _in2, _out)
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
                        test0_84(_in1, _in2, _out)
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
                        test0_82(_in1, _in2, _out)
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
                        test0_84(_in1, _in2, _out)
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
                        test0_90(_in1, _in2, _out)
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
                        test0_90(_in1, _in2, _out)
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
                        test0_90(_in1, _in2, _out)
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
                        test0_90(_in1, _in2, _out)
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
                        test0_92(_in1, _in2, _out)
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
                        test0_92(_in1, _in2, _out)
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
                        test0_92(_in1, _in2, _out)
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
                        test0_92(_in1, _in2, _out)
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
                        test0_94(_in1, _in2, _out)
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
                        test0_96(_in1, _in2, _out)
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
                        test0_98(_in1, _in2, _out)
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
                        test0_100(_in1, _in2, _out)
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
                        test0_98(_in1, _in2, _out)
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
                        test0_100(_in1, _in2, _out)
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
                        test0_102(_in1, _in2, _out)
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
                        test0_104(_in1, _in2, _out)
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
                        test0_102(_in1, _in2, _out)
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
                        test0_104(_in1, _in2, _out)
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
                        test0_92(_in1, _in2, _out)
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
                        test0_92(_in1, _in2, _out)
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
                        test0_92(_in1, _in2, _out)
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
                        test0_92(_in1, _in2, _out)
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
                        test0_90(_in1, _in2, _out)
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
                        test0_90(_in1, _in2, _out)
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
                        test0_90(_in1, _in2, _out)
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
                        test0_90(_in1, _in2, _out)
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
                        test0_96(_in1, _in2, _out)
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
                        test0_94(_in1, _in2, _out)
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
                        test0_102(_in1, _in2, _out)
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
                        test0_104(_in1, _in2, _out)
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
                        test0_102(_in1, _in2, _out)
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
                        test0_104(_in1, _in2, _out)
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
                        test0_98(_in1, _in2, _out)
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
                        test0_100(_in1, _in2, _out)
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
                        test0_98(_in1, _in2, _out)
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
                        test0_100(_in1, _in2, _out)
                    }) as i32))
                    != 0;
                (*accumulate.borrow_mut()) = rhs_0
            };
        }
    }
    return (*accumulate.borrow());
}
pub fn test_limits_119() -> bool {
    assert!(
        (({
            let ___m: u32 = (u32::MAX as u32).wrapping_sub(1_u32);
            let ___n: u32 = (u32::MAX as u32).wrapping_sub(1_u32);
            lcm_21(___m, ___n)
        }) == (u32::MAX as u32).wrapping_sub(1_u32))
    );
    assert!(
        (({
            let ___m: u32 = u32::MAX;
            let ___n: u32 = u32::MAX;
            lcm_21(___m, ___n)
        }) == u32::MAX)
    );
    return true;
}
pub fn test_limits_120() -> bool {
    assert!(
        (({
            let ___m: u64 = (u64::MAX as u64).wrapping_sub(1_u64);
            let ___n: u64 = (u64::MAX as u64).wrapping_sub(1_u64);
            lcm_29(___m, ___n)
        }) == (u64::MAX as u64).wrapping_sub(1_u64))
    );
    assert!(
        (({
            let ___m: u64 = u64::MAX;
            let ___n: u64 = u64::MAX;
            lcm_29(___m, ___n)
        }) == u64::MAX)
    );
    return true;
}
pub fn test_limits_121() -> bool {
    assert!(
        (({
            let ___m: i32 = (i32::MAX - 1);
            let ___n: i32 = (i32::MAX - 1);
            lcm_19(___m, ___n)
        }) == (i32::MAX - 1))
    );
    assert!(
        (({
            let ___m: i32 = i32::MAX;
            let ___n: i32 = i32::MAX;
            lcm_19(___m, ___n)
        }) == i32::MAX)
    );
    return true;
}
pub fn test_limits_122() -> bool {
    assert!(
        (({
            let ___m: i64 = (i64::MAX - 1_i64);
            let ___n: i64 = (i64::MAX - 1_i64);
            lcm_27(___m, ___n)
        }) == (i64::MAX - 1_i64))
    );
    assert!(
        (({
            let ___m: i64 = i64::MAX;
            let ___n: i64 = i64::MAX;
            lcm_27(___m, ___n)
        }) == i64::MAX)
    );
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
    assert!(({ do_test_106(Some(non_cce),) }));
    assert!(({ do_test_107(Some(non_cce),) }));
    assert!(({ do_test_108(Some(non_cce),) }));
    assert!(({ do_test_109(Some(non_cce),) }));
    assert!(({ do_test_110(Some(non_cce),) }));
    const _: () = assert!(true, "do_test<std::int8_t>()");;
    const _: () = assert!(true, "do_test<std::int16_t>()");;
    const _: () = assert!(true, "do_test<std::int32_t>()");;
    const _: () = assert!(true, "do_test<std::int64_t>()");;
    assert!(({ do_test_106(Some(non_cce),) }));
    assert!(({ do_test_107(Some(non_cce),) }));
    assert!(({ do_test_108(Some(non_cce),) }));
    assert!(({ do_test_109(Some(non_cce),) }));
    const _: () = assert!(true, "do_test<signed char, int>()");;
    const _: () = assert!(true, "do_test<int, signed char>()");;
    const _: () = assert!(true, "do_test<short, int>()");;
    const _: () = assert!(true, "do_test<int, short>()");;
    const _: () = assert!(true, "do_test<int, long>()");;
    const _: () = assert!(true, "do_test<long, int>()");;
    const _: () = assert!(true, "do_test<int, long long>()");;
    const _: () = assert!(true, "do_test<long long, int>()");;
    assert!(({ do_test_111(Some(non_cce),) }));
    assert!(({ do_test_112(Some(non_cce),) }));
    assert!(({ do_test_113(Some(non_cce),) }));
    assert!(({ do_test_114(Some(non_cce),) }));
    assert!(({ do_test_115(Some(non_cce),) }));
    assert!(({ do_test_116(Some(non_cce),) }));
    assert!(({ do_test_117(Some(non_cce),) }));
    assert!(({ do_test_118(Some(non_cce),) }));
    {
        let res1: Value<i64> = Rc::new(RefCell::new(({ lcm_77(1234_i64, (-2147483647_i32 - 1)) })));
        &({ lcm_83((-2147483647_i32 - 1), 2_u64) });
        const _: () = assert!(true, "std::is_same_v<decltype(res1), std::int64_t>");;
        assert!(({ ((*res1.borrow()) == 1324997410816_i64) }));
    }
    {
        assert!(({ test_limits_119() }));
        assert!(({ test_limits_119() }));
        assert!(({ test_limits_120() }));
        assert!(({ test_limits_121() }));
        assert!(({ test_limits_121() }));
        assert!(({ test_limits_122() }));
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
