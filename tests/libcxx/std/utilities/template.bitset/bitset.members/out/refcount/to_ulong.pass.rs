extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_to_ulong_0() {
    let mut M: usize =
        (if ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) < (0_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            0_u64
        } as usize);
    let mut is_M_zero: bool = unimplemented!();
    let mut X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: Value<usize> = Rc::new(RefCell::new(if is_M_zero {
        0_usize
    } else {
        ((u64::MAX as usize) >> X)
    }));
    let tests: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([
        0_usize,
        ({
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(1_u64));
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_0.as_pointer();
                let __b = __tmp_1.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_2: Value<u64> = Rc::new(RefCell::new(2_u64));
            let __tmp_3: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_2.as_pointer();
                let __b = __tmp_3.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_4: Value<u64> = Rc::new(RefCell::new(3_u64));
            let __tmp_5: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_4.as_pointer();
                let __b = __tmp_5.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_6: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_7: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(3_usize) as u64)));
            ({
                let __a = __tmp_6.as_pointer();
                let __b = __tmp_7.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_8: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_9: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(2_usize) as u64)));
            ({
                let __a = __tmp_8.as_pointer();
                let __b = __tmp_9.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_10: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_11: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(1_usize) as u64)));
            ({
                let __a = __tmp_10.as_pointer();
                let __b = __tmp_11.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<usize>),
        (tests.as_pointer() as Ptr<usize>).offset((8_i64) as isize),
    ) {
        let mut j: usize = (j.read());
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((j == (unimplemented!() as usize)));
    }
    {
        let mut val: u64 = 1526726565_u64;
        let mut canFit: bool =
            ((0_u64 as usize) < (::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 0_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == (val & mask)));
    }
}
pub fn test_to_ulong_1() {
    let mut M: usize =
        (if ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) < (1_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            1_u64
        } as usize);
    let mut is_M_zero: bool = unimplemented!();
    let mut X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: Value<usize> = Rc::new(RefCell::new(if is_M_zero {
        0_usize
    } else {
        ((u64::MAX as usize) >> X)
    }));
    let tests: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([
        0_usize,
        ({
            let __tmp_12: Value<u64> = Rc::new(RefCell::new(1_u64));
            let __tmp_13: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_12.as_pointer();
                let __b = __tmp_13.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_14: Value<u64> = Rc::new(RefCell::new(2_u64));
            let __tmp_15: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_14.as_pointer();
                let __b = __tmp_15.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_16: Value<u64> = Rc::new(RefCell::new(3_u64));
            let __tmp_17: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_16.as_pointer();
                let __b = __tmp_17.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_18: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_19: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(3_usize) as u64)));
            ({
                let __a = __tmp_18.as_pointer();
                let __b = __tmp_19.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_20: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_21: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(2_usize) as u64)));
            ({
                let __a = __tmp_20.as_pointer();
                let __b = __tmp_21.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_22: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_23: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(1_usize) as u64)));
            ({
                let __a = __tmp_22.as_pointer();
                let __b = __tmp_23.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<usize>),
        (tests.as_pointer() as Ptr<usize>).offset((8_i64) as isize),
    ) {
        let mut j: usize = (j.read());
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((j == (unimplemented!() as usize)));
    }
    {
        let mut val: u64 = 1526726565_u64;
        let mut canFit: bool =
            ((1_u64 as usize) < (::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 1_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == (val & mask)));
    }
}
pub fn test_to_ulong_2() {
    let mut M: usize =
        (if ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) < (31_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            31_u64
        } as usize);
    let mut is_M_zero: bool = unimplemented!();
    let mut X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: Value<usize> = Rc::new(RefCell::new(if is_M_zero {
        0_usize
    } else {
        ((u64::MAX as usize) >> X)
    }));
    let tests: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([
        0_usize,
        ({
            let __tmp_24: Value<u64> = Rc::new(RefCell::new(1_u64));
            let __tmp_25: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_24.as_pointer();
                let __b = __tmp_25.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_26: Value<u64> = Rc::new(RefCell::new(2_u64));
            let __tmp_27: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_26.as_pointer();
                let __b = __tmp_27.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_28: Value<u64> = Rc::new(RefCell::new(3_u64));
            let __tmp_29: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_28.as_pointer();
                let __b = __tmp_29.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_30: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_31: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(3_usize) as u64)));
            ({
                let __a = __tmp_30.as_pointer();
                let __b = __tmp_31.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_32: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_33: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(2_usize) as u64)));
            ({
                let __a = __tmp_32.as_pointer();
                let __b = __tmp_33.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_34: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_35: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(1_usize) as u64)));
            ({
                let __a = __tmp_34.as_pointer();
                let __b = __tmp_35.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<usize>),
        (tests.as_pointer() as Ptr<usize>).offset((8_i64) as isize),
    ) {
        let mut j: usize = (j.read());
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((j == (unimplemented!() as usize)));
    }
    {
        let mut val: u64 = 1526726565_u64;
        let mut canFit: bool =
            ((31_u64 as usize) < (::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 31_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == (val & mask)));
    }
}
pub fn test_to_ulong_3() {
    let mut M: usize =
        (if ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) < (32_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            32_u64
        } as usize);
    let mut is_M_zero: bool = unimplemented!();
    let mut X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: Value<usize> = Rc::new(RefCell::new(if is_M_zero {
        0_usize
    } else {
        ((u64::MAX as usize) >> X)
    }));
    let tests: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([
        0_usize,
        ({
            let __tmp_36: Value<u64> = Rc::new(RefCell::new(1_u64));
            let __tmp_37: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_36.as_pointer();
                let __b = __tmp_37.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_38: Value<u64> = Rc::new(RefCell::new(2_u64));
            let __tmp_39: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_38.as_pointer();
                let __b = __tmp_39.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_40: Value<u64> = Rc::new(RefCell::new(3_u64));
            let __tmp_41: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_40.as_pointer();
                let __b = __tmp_41.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_42: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_43: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(3_usize) as u64)));
            ({
                let __a = __tmp_42.as_pointer();
                let __b = __tmp_43.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_44: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_45: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(2_usize) as u64)));
            ({
                let __a = __tmp_44.as_pointer();
                let __b = __tmp_45.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_46: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_47: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(1_usize) as u64)));
            ({
                let __a = __tmp_46.as_pointer();
                let __b = __tmp_47.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<usize>),
        (tests.as_pointer() as Ptr<usize>).offset((8_i64) as isize),
    ) {
        let mut j: usize = (j.read());
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((j == (unimplemented!() as usize)));
    }
    {
        let mut val: u64 = 1526726565_u64;
        let mut canFit: bool =
            ((32_u64 as usize) < (::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 32_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == (val & mask)));
    }
}
pub fn test_to_ulong_4() {
    let mut M: usize =
        (if ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) < (33_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            33_u64
        } as usize);
    let mut is_M_zero: bool = unimplemented!();
    let mut X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: Value<usize> = Rc::new(RefCell::new(if is_M_zero {
        0_usize
    } else {
        ((u64::MAX as usize) >> X)
    }));
    let tests: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([
        0_usize,
        ({
            let __tmp_48: Value<u64> = Rc::new(RefCell::new(1_u64));
            let __tmp_49: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_48.as_pointer();
                let __b = __tmp_49.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_50: Value<u64> = Rc::new(RefCell::new(2_u64));
            let __tmp_51: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_50.as_pointer();
                let __b = __tmp_51.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_52: Value<u64> = Rc::new(RefCell::new(3_u64));
            let __tmp_53: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_52.as_pointer();
                let __b = __tmp_53.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_54: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_55: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(3_usize) as u64)));
            ({
                let __a = __tmp_54.as_pointer();
                let __b = __tmp_55.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_56: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_57: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(2_usize) as u64)));
            ({
                let __a = __tmp_56.as_pointer();
                let __b = __tmp_57.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_58: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_59: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(1_usize) as u64)));
            ({
                let __a = __tmp_58.as_pointer();
                let __b = __tmp_59.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<usize>),
        (tests.as_pointer() as Ptr<usize>).offset((8_i64) as isize),
    ) {
        let mut j: usize = (j.read());
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((j == (unimplemented!() as usize)));
    }
    {
        let mut val: u64 = 1526726565_u64;
        let mut canFit: bool =
            ((33_u64 as usize) < (::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 33_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == (val & mask)));
    }
}
pub fn test_to_ulong_5() {
    let mut M: usize =
        (if ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) < (63_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            63_u64
        } as usize);
    let mut is_M_zero: bool = unimplemented!();
    let mut X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: Value<usize> = Rc::new(RefCell::new(if is_M_zero {
        0_usize
    } else {
        ((u64::MAX as usize) >> X)
    }));
    let tests: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([
        0_usize,
        ({
            let __tmp_60: Value<u64> = Rc::new(RefCell::new(1_u64));
            let __tmp_61: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_60.as_pointer();
                let __b = __tmp_61.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_62: Value<u64> = Rc::new(RefCell::new(2_u64));
            let __tmp_63: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_62.as_pointer();
                let __b = __tmp_63.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_64: Value<u64> = Rc::new(RefCell::new(3_u64));
            let __tmp_65: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_64.as_pointer();
                let __b = __tmp_65.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_66: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_67: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(3_usize) as u64)));
            ({
                let __a = __tmp_66.as_pointer();
                let __b = __tmp_67.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_68: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_69: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(2_usize) as u64)));
            ({
                let __a = __tmp_68.as_pointer();
                let __b = __tmp_69.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_70: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_71: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(1_usize) as u64)));
            ({
                let __a = __tmp_70.as_pointer();
                let __b = __tmp_71.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<usize>),
        (tests.as_pointer() as Ptr<usize>).offset((8_i64) as isize),
    ) {
        let mut j: usize = (j.read());
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((j == (unimplemented!() as usize)));
    }
    {
        let mut val: u64 = 1526726565_u64;
        let mut canFit: bool =
            ((63_u64 as usize) < (::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 63_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == (val & mask)));
    }
}
pub fn test_to_ulong_6() {
    let mut M: usize =
        (if ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) < (64_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            64_u64
        } as usize);
    let mut is_M_zero: bool = unimplemented!();
    let mut X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: Value<usize> = Rc::new(RefCell::new(if is_M_zero {
        0_usize
    } else {
        ((u64::MAX as usize) >> X)
    }));
    let tests: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([
        0_usize,
        ({
            let __tmp_72: Value<u64> = Rc::new(RefCell::new(1_u64));
            let __tmp_73: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_72.as_pointer();
                let __b = __tmp_73.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_74: Value<u64> = Rc::new(RefCell::new(2_u64));
            let __tmp_75: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_74.as_pointer();
                let __b = __tmp_75.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_76: Value<u64> = Rc::new(RefCell::new(3_u64));
            let __tmp_77: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_76.as_pointer();
                let __b = __tmp_77.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_78: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_79: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(3_usize) as u64)));
            ({
                let __a = __tmp_78.as_pointer();
                let __b = __tmp_79.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_80: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_81: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(2_usize) as u64)));
            ({
                let __a = __tmp_80.as_pointer();
                let __b = __tmp_81.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_82: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_83: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(1_usize) as u64)));
            ({
                let __a = __tmp_82.as_pointer();
                let __b = __tmp_83.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<usize>),
        (tests.as_pointer() as Ptr<usize>).offset((8_i64) as isize),
    ) {
        let mut j: usize = (j.read());
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((j == (unimplemented!() as usize)));
    }
    {
        let mut val: u64 = 1526726565_u64;
        let mut canFit: bool =
            ((64_u64 as usize) < (::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 64_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == (val & mask)));
    }
}
pub fn test_to_ulong_7() {
    let mut M: usize =
        (if ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) < (65_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            65_u64
        } as usize);
    let mut is_M_zero: bool = unimplemented!();
    let mut X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: Value<usize> = Rc::new(RefCell::new(if is_M_zero {
        0_usize
    } else {
        ((u64::MAX as usize) >> X)
    }));
    let tests: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([
        0_usize,
        ({
            let __tmp_84: Value<u64> = Rc::new(RefCell::new(1_u64));
            let __tmp_85: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_84.as_pointer();
                let __b = __tmp_85.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_86: Value<u64> = Rc::new(RefCell::new(2_u64));
            let __tmp_87: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_86.as_pointer();
                let __b = __tmp_87.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_88: Value<u64> = Rc::new(RefCell::new(3_u64));
            let __tmp_89: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_88.as_pointer();
                let __b = __tmp_89.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_90: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_91: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(3_usize) as u64)));
            ({
                let __a = __tmp_90.as_pointer();
                let __b = __tmp_91.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_92: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_93: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(2_usize) as u64)));
            ({
                let __a = __tmp_92.as_pointer();
                let __b = __tmp_93.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_94: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_95: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(1_usize) as u64)));
            ({
                let __a = __tmp_94.as_pointer();
                let __b = __tmp_95.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<usize>),
        (tests.as_pointer() as Ptr<usize>).offset((8_i64) as isize),
    ) {
        let mut j: usize = (j.read());
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((j == (unimplemented!() as usize)));
    }
    {
        let mut val: u64 = 1526726565_u64;
        let mut canFit: bool =
            ((65_u64 as usize) < (::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 65_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == (val & mask)));
    }
}
pub fn test_to_ulong_8() {
    let mut M: usize =
        (if ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) < (1000_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            1000_u64
        } as usize);
    let mut is_M_zero: bool = unimplemented!();
    let mut X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: Value<usize> = Rc::new(RefCell::new(if is_M_zero {
        0_usize
    } else {
        ((u64::MAX as usize) >> X)
    }));
    let tests: Value<Box<[usize]>> = Rc::new(RefCell::new(Box::new([
        0_usize,
        ({
            let __tmp_96: Value<u64> = Rc::new(RefCell::new(1_u64));
            let __tmp_97: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_96.as_pointer();
                let __b = __tmp_97.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_98: Value<u64> = Rc::new(RefCell::new(2_u64));
            let __tmp_99: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_98.as_pointer();
                let __b = __tmp_99.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_100: Value<u64> = Rc::new(RefCell::new(3_u64));
            let __tmp_101: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            ({
                let __a = __tmp_100.as_pointer();
                let __b = __tmp_101.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_102: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_103: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(3_usize) as u64)));
            ({
                let __a = __tmp_102.as_pointer();
                let __b = __tmp_103.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_104: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_105: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(2_usize) as u64)));
            ({
                let __a = __tmp_104.as_pointer();
                let __b = __tmp_105.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        ({
            let __tmp_106: Value<u64> = Rc::new(RefCell::new(((*max.borrow()) as u64)));
            let __tmp_107: Value<u64> =
                Rc::new(RefCell::new(((*max.borrow()).wrapping_sub(1_usize) as u64)));
            ({
                let __a = __tmp_106.as_pointer();
                let __b = __tmp_107.as_pointer();
                if CppLess::less(&__b, &__a) { __b } else { __a }
            }
            .read())
        } as usize),
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<usize>),
        (tests.as_pointer() as Ptr<usize>).offset((8_i64) as isize),
    ) {
        let mut j: usize = (j.read());
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((j == (unimplemented!() as usize)));
    }
    {
        let mut val: u64 = 1526726565_u64;
        let mut canFit: bool =
            ((1000_u64 as usize) < (::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 1000_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        assert!((unimplemented!() == (val & mask)));
    }
}
pub fn test_9() -> bool {
    ({ test_to_ulong_0() });
    ({ test_to_ulong_1() });
    ({ test_to_ulong_2() });
    ({ test_to_ulong_3() });
    ({ test_to_ulong_4() });
    ({ test_to_ulong_5() });
    ({ test_to_ulong_6() });
    ({ test_to_ulong_7() });
    ({ test_to_ulong_8() });
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_9() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
