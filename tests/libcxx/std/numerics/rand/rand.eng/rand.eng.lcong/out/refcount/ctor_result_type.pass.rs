extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn make_implicit_0() -> () {
    return unimplemented!();
}
pub fn test_convertible_imp_1(mut _a0: i32) -> bool {
    return true;
}
pub fn test_convertible_imp_2(mut _a0: i64) -> bool {
    return false;
}
pub fn test_convertible_3() -> bool {
    return ({ test_convertible_imp_1(0) });
}
pub fn test_convertible_4() -> bool {
    return ({ test_convertible_imp_2(0_i64) });
}
pub fn to_string_5(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_6(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_7(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_8(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_9(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_10(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_11(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_12(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_13(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_14(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_15(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_16(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_17(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_18(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_19(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_20(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_21(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_22(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_23(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_24(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_25(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_26(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_27(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn to_string_28(e: Ptr<()>) -> Vec<i8> {
    let os: Value<()> = Rc::new(RefCell::new(unimplemented!()));
    unimplemented!();
    return unimplemented!();
}
pub fn test1_29() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_5(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_0,) => {
                    Ptr::<i8>::from_string_literal(b"5").with_c_str(|__s| {
                        (*__tmp_0.upgrade().deref())
                            [..(*__tmp_0.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_6(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_1,) => {
                    Ptr::<i8>::from_string_literal(b"5").with_c_str(|__s| {
                        (*__tmp_1.upgrade().deref())
                            [..(*__tmp_1.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_7(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_2,) => {
                    Ptr::<i8>::from_string_literal(b"1").with_c_str(|__s| {
                        (*__tmp_2.upgrade().deref())
                            [..(*__tmp_2.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
}
pub fn test1_30() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_8(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_3,) => {
                    Ptr::<i8>::from_string_literal(b"5").with_c_str(|__s| {
                        (*__tmp_3.upgrade().deref())
                            [..(*__tmp_3.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_9(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_4,) => {
                    Ptr::<i8>::from_string_literal(b"5").with_c_str(|__s| {
                        (*__tmp_4.upgrade().deref())
                            [..(*__tmp_4.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_10(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_5,) => {
                    Ptr::<i8>::from_string_literal(b"1").with_c_str(|__s| {
                        (*__tmp_5.upgrade().deref())
                            [..(*__tmp_5.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
}
pub fn test1_31() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_11(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_6,) => {
                    Ptr::<i8>::from_string_literal(b"5").with_c_str(|__s| {
                        (*__tmp_6.upgrade().deref())
                            [..(*__tmp_6.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_12(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_7,) => {
                    Ptr::<i8>::from_string_literal(b"5").with_c_str(|__s| {
                        (*__tmp_7.upgrade().deref())
                            [..(*__tmp_7.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_13(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_8,) => {
                    Ptr::<i8>::from_string_literal(b"1").with_c_str(|__s| {
                        (*__tmp_8.upgrade().deref())
                            [..(*__tmp_8.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
}
pub fn test1_32() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_14(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_9,) => {
                    Ptr::<i8>::from_string_literal(b"5").with_c_str(|__s| {
                        (*__tmp_9.upgrade().deref())
                            [..(*__tmp_9.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_15(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_10,) => {
                    Ptr::<i8>::from_string_literal(b"5").with_c_str(|__s| {
                        (*__tmp_10.upgrade().deref())
                            [..(*__tmp_10.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_16(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_11,) => {
                    Ptr::<i8>::from_string_literal(b"1").with_c_str(|__s| {
                        (*__tmp_11.upgrade().deref())
                            [..(*__tmp_11.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
}
pub fn test2_33() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_5(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_12,) => {
                    Ptr::<i8>::from_string_literal(b"0").with_c_str(|__s| {
                        (*__tmp_12.upgrade().deref())
                            [..(*__tmp_12.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_6(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_13,) => {
                    Ptr::<i8>::from_string_literal(b"0").with_c_str(|__s| {
                        (*__tmp_13.upgrade().deref())
                            [..(*__tmp_13.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_7(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_14,) => {
                    Ptr::<i8>::from_string_literal(b"0").with_c_str(|__s| {
                        (*__tmp_14.upgrade().deref())
                            [..(*__tmp_14.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
}
pub fn test2_34() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_8(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_15,) => {
                    Ptr::<i8>::from_string_literal(b"0").with_c_str(|__s| {
                        (*__tmp_15.upgrade().deref())
                            [..(*__tmp_15.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_9(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_16,) => {
                    Ptr::<i8>::from_string_literal(b"0").with_c_str(|__s| {
                        (*__tmp_16.upgrade().deref())
                            [..(*__tmp_16.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_10(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_17,) => {
                    Ptr::<i8>::from_string_literal(b"0").with_c_str(|__s| {
                        (*__tmp_17.upgrade().deref())
                            [..(*__tmp_17.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
}
pub fn test2_35() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_11(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_18,) => {
                    Ptr::<i8>::from_string_literal(b"0").with_c_str(|__s| {
                        (*__tmp_18.upgrade().deref())
                            [..(*__tmp_18.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_12(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_19,) => {
                    Ptr::<i8>::from_string_literal(b"0").with_c_str(|__s| {
                        (*__tmp_19.upgrade().deref())
                            [..(*__tmp_19.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_13(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_20,) => {
                    Ptr::<i8>::from_string_literal(b"0").with_c_str(|__s| {
                        (*__tmp_20.upgrade().deref())
                            [..(*__tmp_20.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
}
pub fn test2_36() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_14(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_21,) => {
                    Ptr::<i8>::from_string_literal(b"0").with_c_str(|__s| {
                        (*__tmp_21.upgrade().deref())
                            [..(*__tmp_21.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_15(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_22,) => {
                    Ptr::<i8>::from_string_literal(b"0").with_c_str(|__s| {
                        (*__tmp_22.upgrade().deref())
                            [..(*__tmp_22.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_16(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_23,) => {
                    Ptr::<i8>::from_string_literal(b"0").with_c_str(|__s| {
                        (*__tmp_23.upgrade().deref())
                            [..(*__tmp_23.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
}
pub fn test3_37() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_17(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_24,) => {
                    Ptr::<i8>::from_string_literal(b"3").with_c_str(|__s| {
                        (*__tmp_24.upgrade().deref())
                            [..(*__tmp_24.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_18(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_25,) => {
                    Ptr::<i8>::from_string_literal(b"5").with_c_str(|__s| {
                        (*__tmp_25.upgrade().deref())
                            [..(*__tmp_25.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_19(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_26,) => {
                    Ptr::<i8>::from_string_literal(b"3").with_c_str(|__s| {
                        (*__tmp_26.upgrade().deref())
                            [..(*__tmp_26.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
}
pub fn test3_38() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_20(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_27,) => {
                    Ptr::<i8>::from_string_literal(b"3").with_c_str(|__s| {
                        (*__tmp_27.upgrade().deref())
                            [..(*__tmp_27.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_21(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_28,) => {
                    Ptr::<i8>::from_string_literal(b"5").with_c_str(|__s| {
                        (*__tmp_28.upgrade().deref())
                            [..(*__tmp_28.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_22(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_29,) => {
                    Ptr::<i8>::from_string_literal(b"3").with_c_str(|__s| {
                        (*__tmp_29.upgrade().deref())
                            [..(*__tmp_29.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
}
pub fn test3_39() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_23(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_30,) => {
                    Ptr::<i8>::from_string_literal(b"3").with_c_str(|__s| {
                        (*__tmp_30.upgrade().deref())
                            [..(*__tmp_30.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_24(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_31,) => {
                    Ptr::<i8>::from_string_literal(b"5").with_c_str(|__s| {
                        (*__tmp_31.upgrade().deref())
                            [..(*__tmp_31.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_25(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_32,) => {
                    Ptr::<i8>::from_string_literal(b"3").with_c_str(|__s| {
                        (*__tmp_32.upgrade().deref())
                            [..(*__tmp_32.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
}
pub fn test3_40() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_26(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_33,) => {
                    Ptr::<i8>::from_string_literal(b"3").with_c_str(|__s| {
                        (*__tmp_33.upgrade().deref())
                            [..(*__tmp_33.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_27(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_34,) => {
                    Ptr::<i8>::from_string_literal(b"5").with_c_str(|__s| {
                        (*__tmp_34.upgrade().deref())
                            [..(*__tmp_34.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_28(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_35,) => {
                    Ptr::<i8>::from_string_literal(b"3").with_c_str(|__s| {
                        (*__tmp_35.upgrade().deref())
                            [..(*__tmp_35.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
}
pub fn test4_41() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_17(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_36,) => {
                    Ptr::<i8>::from_string_literal(b"1").with_c_str(|__s| {
                        (*__tmp_36.upgrade().deref())
                            [..(*__tmp_36.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_18(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_37,) => {
                    Ptr::<i8>::from_string_literal(b"1").with_c_str(|__s| {
                        (*__tmp_37.upgrade().deref())
                            [..(*__tmp_37.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_19(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_38,) => {
                    Ptr::<i8>::from_string_literal(b"1").with_c_str(|__s| {
                        (*__tmp_38.upgrade().deref())
                            [..(*__tmp_38.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
}
pub fn test4_42() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_20(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_39,) => {
                    Ptr::<i8>::from_string_literal(b"1").with_c_str(|__s| {
                        (*__tmp_39.upgrade().deref())
                            [..(*__tmp_39.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_21(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_40,) => {
                    Ptr::<i8>::from_string_literal(b"1").with_c_str(|__s| {
                        (*__tmp_40.upgrade().deref())
                            [..(*__tmp_40.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_22(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_41,) => {
                    Ptr::<i8>::from_string_literal(b"1").with_c_str(|__s| {
                        (*__tmp_41.upgrade().deref())
                            [..(*__tmp_41.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
}
pub fn test4_43() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_23(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_42,) => {
                    Ptr::<i8>::from_string_literal(b"1").with_c_str(|__s| {
                        (*__tmp_42.upgrade().deref())
                            [..(*__tmp_42.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_24(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_43,) => {
                    Ptr::<i8>::from_string_literal(b"1").with_c_str(|__s| {
                        (*__tmp_43.upgrade().deref())
                            [..(*__tmp_43.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_25(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_44,) => {
                    Ptr::<i8>::from_string_literal(b"1").with_c_str(|__s| {
                        (*__tmp_44.upgrade().deref())
                            [..(*__tmp_44.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
}
pub fn test4_44() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_26(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_45,) => {
                    Ptr::<i8>::from_string_literal(b"1").with_c_str(|__s| {
                        (*__tmp_45.upgrade().deref())
                            [..(*__tmp_45.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_27(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_46,) => {
                    Ptr::<i8>::from_string_literal(b"1").with_c_str(|__s| {
                        (*__tmp_46.upgrade().deref())
                            [..(*__tmp_46.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            match (Rc::new(RefCell::new(({ to_string_28(e.as_pointer(),) }))).as_pointer(),) {
                (__tmp_47,) => {
                    Ptr::<i8>::from_string_literal(b"1").with_c_str(|__s| {
                        (*__tmp_47.upgrade().deref())
                            [..(*__tmp_47.upgrade().deref()).len().saturating_sub(1)]
                            == *__s
                    })
                }
            }
        );
    }
}
pub fn test_implicit_45() {
    const _: () = assert!(true, "test_convertible<E>()");;
    assert!(unimplemented!());
    const _: () = assert!(true, "!test_convertible<E, T>()");;
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test1_29() });
    ({ test1_30() });
    ({ test1_31() });
    ({ test1_32() });
    ({ test2_33() });
    ({ test2_34() });
    ({ test2_35() });
    ({ test2_36() });
    ({ test3_37() });
    ({ test3_38() });
    ({ test3_39() });
    ({ test3_40() });
    ({ test4_41() });
    ({ test4_42() });
    ({ test4_43() });
    ({ test4_44() });
    ({ test_implicit_45() });
    return 0;
}
pub fn __cpp2rust_init_globals() {}
