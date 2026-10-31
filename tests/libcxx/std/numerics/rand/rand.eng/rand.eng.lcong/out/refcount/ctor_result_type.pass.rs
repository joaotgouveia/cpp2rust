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
            Ptr::<i8>::from_string_literal(b"5")
                .with_c_str(|__s| ({ to_string_5(e.as_pointer(),) })
                    [..({ to_string_5(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"5")
                .with_c_str(|__s| ({ to_string_6(e.as_pointer(),) })
                    [..({ to_string_6(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"1")
                .with_c_str(|__s| ({ to_string_7(e.as_pointer(),) })
                    [..({ to_string_7(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
}
pub fn test1_30() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"5")
                .with_c_str(|__s| ({ to_string_8(e.as_pointer(),) })
                    [..({ to_string_8(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"5")
                .with_c_str(|__s| ({ to_string_9(e.as_pointer(),) })
                    [..({ to_string_9(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"1")
                .with_c_str(|__s| ({ to_string_10(e.as_pointer(),) })
                    [..({ to_string_10(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
}
pub fn test1_31() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"5")
                .with_c_str(|__s| ({ to_string_11(e.as_pointer(),) })
                    [..({ to_string_11(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"5")
                .with_c_str(|__s| ({ to_string_12(e.as_pointer(),) })
                    [..({ to_string_12(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"1")
                .with_c_str(|__s| ({ to_string_13(e.as_pointer(),) })
                    [..({ to_string_13(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
}
pub fn test1_32() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"5")
                .with_c_str(|__s| ({ to_string_14(e.as_pointer(),) })
                    [..({ to_string_14(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"5")
                .with_c_str(|__s| ({ to_string_15(e.as_pointer(),) })
                    [..({ to_string_15(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"1")
                .with_c_str(|__s| ({ to_string_16(e.as_pointer(),) })
                    [..({ to_string_16(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
}
pub fn test2_33() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"0")
                .with_c_str(|__s| ({ to_string_5(e.as_pointer(),) })
                    [..({ to_string_5(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"0")
                .with_c_str(|__s| ({ to_string_6(e.as_pointer(),) })
                    [..({ to_string_6(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"0")
                .with_c_str(|__s| ({ to_string_7(e.as_pointer(),) })
                    [..({ to_string_7(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
}
pub fn test2_34() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"0")
                .with_c_str(|__s| ({ to_string_8(e.as_pointer(),) })
                    [..({ to_string_8(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"0")
                .with_c_str(|__s| ({ to_string_9(e.as_pointer(),) })
                    [..({ to_string_9(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"0")
                .with_c_str(|__s| ({ to_string_10(e.as_pointer(),) })
                    [..({ to_string_10(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
}
pub fn test2_35() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"0")
                .with_c_str(|__s| ({ to_string_11(e.as_pointer(),) })
                    [..({ to_string_11(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"0")
                .with_c_str(|__s| ({ to_string_12(e.as_pointer(),) })
                    [..({ to_string_12(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"0")
                .with_c_str(|__s| ({ to_string_13(e.as_pointer(),) })
                    [..({ to_string_13(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
}
pub fn test2_36() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"0")
                .with_c_str(|__s| ({ to_string_14(e.as_pointer(),) })
                    [..({ to_string_14(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"0")
                .with_c_str(|__s| ({ to_string_15(e.as_pointer(),) })
                    [..({ to_string_15(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"0")
                .with_c_str(|__s| ({ to_string_16(e.as_pointer(),) })
                    [..({ to_string_16(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
}
pub fn test3_37() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"3")
                .with_c_str(|__s| ({ to_string_17(e.as_pointer(),) })
                    [..({ to_string_17(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"5")
                .with_c_str(|__s| ({ to_string_18(e.as_pointer(),) })
                    [..({ to_string_18(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"3")
                .with_c_str(|__s| ({ to_string_19(e.as_pointer(),) })
                    [..({ to_string_19(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
}
pub fn test3_38() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"3")
                .with_c_str(|__s| ({ to_string_20(e.as_pointer(),) })
                    [..({ to_string_20(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"5")
                .with_c_str(|__s| ({ to_string_21(e.as_pointer(),) })
                    [..({ to_string_21(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"3")
                .with_c_str(|__s| ({ to_string_22(e.as_pointer(),) })
                    [..({ to_string_22(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
}
pub fn test3_39() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"3")
                .with_c_str(|__s| ({ to_string_23(e.as_pointer(),) })
                    [..({ to_string_23(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"5")
                .with_c_str(|__s| ({ to_string_24(e.as_pointer(),) })
                    [..({ to_string_24(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"3")
                .with_c_str(|__s| ({ to_string_25(e.as_pointer(),) })
                    [..({ to_string_25(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
}
pub fn test3_40() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"3")
                .with_c_str(|__s| ({ to_string_26(e.as_pointer(),) })
                    [..({ to_string_26(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"5")
                .with_c_str(|__s| ({ to_string_27(e.as_pointer(),) })
                    [..({ to_string_27(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"3")
                .with_c_str(|__s| ({ to_string_28(e.as_pointer(),) })
                    [..({ to_string_28(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
}
pub fn test4_41() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"1")
                .with_c_str(|__s| ({ to_string_17(e.as_pointer(),) })
                    [..({ to_string_17(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"1")
                .with_c_str(|__s| ({ to_string_18(e.as_pointer(),) })
                    [..({ to_string_18(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"1")
                .with_c_str(|__s| ({ to_string_19(e.as_pointer(),) })
                    [..({ to_string_19(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
}
pub fn test4_42() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"1")
                .with_c_str(|__s| ({ to_string_20(e.as_pointer(),) })
                    [..({ to_string_20(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"1")
                .with_c_str(|__s| ({ to_string_21(e.as_pointer(),) })
                    [..({ to_string_21(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"1")
                .with_c_str(|__s| ({ to_string_22(e.as_pointer(),) })
                    [..({ to_string_22(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
}
pub fn test4_43() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"1")
                .with_c_str(|__s| ({ to_string_23(e.as_pointer(),) })
                    [..({ to_string_23(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"1")
                .with_c_str(|__s| ({ to_string_24(e.as_pointer(),) })
                    [..({ to_string_24(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"1")
                .with_c_str(|__s| ({ to_string_25(e.as_pointer(),) })
                    [..({ to_string_25(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
}
pub fn test4_44() {
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"1")
                .with_c_str(|__s| ({ to_string_26(e.as_pointer(),) })
                    [..({ to_string_26(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"1")
                .with_c_str(|__s| ({ to_string_27(e.as_pointer(),) })
                    [..({ to_string_27(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
        );
    }
    {
        let e: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!(
            Ptr::<i8>::from_string_literal(b"1")
                .with_c_str(|__s| ({ to_string_28(e.as_pointer(),) })
                    [..({ to_string_28(e.as_pointer(),) }).len().saturating_sub(1)]
                    == *__s)
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
