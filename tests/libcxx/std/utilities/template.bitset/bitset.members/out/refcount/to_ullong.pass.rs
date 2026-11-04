extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_to_ullong_0() {
    let mut M: usize =
        (if ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) < (0_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            0_u64
        } as usize);
    let mut is_M_zero: bool = value_1.with(|rc| *rc.borrow());
    let mut X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: Value<u64> = Rc::new(RefCell::new(if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> X)
    }));
    let tests: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        0_u64,
        {
            let __tmp_0: Value<u64> = Rc::new(RefCell::new(1_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_0.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_0.as_pointer()
            }
            .read())
        },
        {
            let __tmp_1: Value<u64> = Rc::new(RefCell::new(2_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_1.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_1.as_pointer()
            }
            .read())
        },
        {
            let __tmp_2: Value<u64> = Rc::new(RefCell::new(3_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_2.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_2.as_pointer()
            }
            .read())
        },
        {
            let __tmp_3: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(3_u64)));
            (if CppLess::less(&__tmp_3.as_pointer(), &max.as_pointer()) {
                __tmp_3.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        {
            let __tmp_4: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(2_u64)));
            (if CppLess::less(&__tmp_4.as_pointer(), &max.as_pointer()) {
                __tmp_4.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        {
            let __tmp_5: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(1_u64)));
            (if CppLess::less(&__tmp_5.as_pointer(), &max.as_pointer()) {
                __tmp_5.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<u64>),
        (tests.as_pointer() as Ptr<u64>).offset((8_i64) as isize),
    ) {
        let mut j: u64 = (j.read());
        let v: Value<std_bitset_0_> = Rc::new(RefCell::new(std_bitset_0_::new_1({ j })));
        assert!(({ (j == ({ (*v.borrow()).to_ullong() })) }));
    }
    {
        let mut val: u64 = 6172934255747443285_u64;
        let mut canFit: bool =
            ((0_u64 as usize) < (::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 0_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<std_bitset_0_> = Rc::new(RefCell::new(std_bitset_0_::new_1({ val })));
        assert!(({ (({ (*v.borrow()).to_ullong() }) == (val & mask)) }));
    }
}
pub fn test_to_ullong_2() {
    let mut M: usize =
        (if ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) < (1_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            1_u64
        } as usize);
    let mut is_M_zero: bool = value_3.with(|rc| *rc.borrow());
    let mut X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: Value<u64> = Rc::new(RefCell::new(if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> X)
    }));
    let tests: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        0_u64,
        {
            let __tmp_6: Value<u64> = Rc::new(RefCell::new(1_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_6.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_6.as_pointer()
            }
            .read())
        },
        {
            let __tmp_7: Value<u64> = Rc::new(RefCell::new(2_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_7.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_7.as_pointer()
            }
            .read())
        },
        {
            let __tmp_8: Value<u64> = Rc::new(RefCell::new(3_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_8.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_8.as_pointer()
            }
            .read())
        },
        {
            let __tmp_9: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(3_u64)));
            (if CppLess::less(&__tmp_9.as_pointer(), &max.as_pointer()) {
                __tmp_9.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        {
            let __tmp_10: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(2_u64)));
            (if CppLess::less(&__tmp_10.as_pointer(), &max.as_pointer()) {
                __tmp_10.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        {
            let __tmp_11: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(1_u64)));
            (if CppLess::less(&__tmp_11.as_pointer(), &max.as_pointer()) {
                __tmp_11.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<u64>),
        (tests.as_pointer() as Ptr<u64>).offset((8_i64) as isize),
    ) {
        let mut j: u64 = (j.read());
        let v: Value<std_bitset_1_> = Rc::new(RefCell::new(std_bitset_1_::new_1({ j })));
        assert!(({ (j == ({ (*v.borrow()).to_ullong() })) }));
    }
    {
        let mut val: u64 = 6172934255747443285_u64;
        let mut canFit: bool =
            ((1_u64 as usize) < (::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 1_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<std_bitset_1_> = Rc::new(RefCell::new(std_bitset_1_::new_1({ val })));
        assert!(({ (({ (*v.borrow()).to_ullong() }) == (val & mask)) }));
    }
}
pub fn test_to_ullong_4() {
    let mut M: usize =
        (if ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) < (31_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            31_u64
        } as usize);
    let mut is_M_zero: bool = value_3.with(|rc| *rc.borrow());
    let mut X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: Value<u64> = Rc::new(RefCell::new(if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> X)
    }));
    let tests: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        0_u64,
        {
            let __tmp_12: Value<u64> = Rc::new(RefCell::new(1_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_12.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_12.as_pointer()
            }
            .read())
        },
        {
            let __tmp_13: Value<u64> = Rc::new(RefCell::new(2_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_13.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_13.as_pointer()
            }
            .read())
        },
        {
            let __tmp_14: Value<u64> = Rc::new(RefCell::new(3_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_14.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_14.as_pointer()
            }
            .read())
        },
        {
            let __tmp_15: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(3_u64)));
            (if CppLess::less(&__tmp_15.as_pointer(), &max.as_pointer()) {
                __tmp_15.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        {
            let __tmp_16: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(2_u64)));
            (if CppLess::less(&__tmp_16.as_pointer(), &max.as_pointer()) {
                __tmp_16.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        {
            let __tmp_17: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(1_u64)));
            (if CppLess::less(&__tmp_17.as_pointer(), &max.as_pointer()) {
                __tmp_17.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<u64>),
        (tests.as_pointer() as Ptr<u64>).offset((8_i64) as isize),
    ) {
        let mut j: u64 = (j.read());
        let v: Value<std_bitset_31_> = Rc::new(RefCell::new(std_bitset_31_::new_1({ j })));
        assert!(({ (j == ({ (*v.borrow()).to_ullong() })) }));
    }
    {
        let mut val: u64 = 6172934255747443285_u64;
        let mut canFit: bool =
            ((31_u64 as usize) < (::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 31_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<std_bitset_31_> = Rc::new(RefCell::new(std_bitset_31_::new_1({ val })));
        assert!(({ (({ (*v.borrow()).to_ullong() }) == (val & mask)) }));
    }
}
pub fn test_to_ullong_5() {
    let mut M: usize =
        (if ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) < (32_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            32_u64
        } as usize);
    let mut is_M_zero: bool = value_3.with(|rc| *rc.borrow());
    let mut X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: Value<u64> = Rc::new(RefCell::new(if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> X)
    }));
    let tests: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        0_u64,
        {
            let __tmp_18: Value<u64> = Rc::new(RefCell::new(1_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_18.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_18.as_pointer()
            }
            .read())
        },
        {
            let __tmp_19: Value<u64> = Rc::new(RefCell::new(2_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_19.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_19.as_pointer()
            }
            .read())
        },
        {
            let __tmp_20: Value<u64> = Rc::new(RefCell::new(3_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_20.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_20.as_pointer()
            }
            .read())
        },
        {
            let __tmp_21: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(3_u64)));
            (if CppLess::less(&__tmp_21.as_pointer(), &max.as_pointer()) {
                __tmp_21.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        {
            let __tmp_22: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(2_u64)));
            (if CppLess::less(&__tmp_22.as_pointer(), &max.as_pointer()) {
                __tmp_22.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        {
            let __tmp_23: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(1_u64)));
            (if CppLess::less(&__tmp_23.as_pointer(), &max.as_pointer()) {
                __tmp_23.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<u64>),
        (tests.as_pointer() as Ptr<u64>).offset((8_i64) as isize),
    ) {
        let mut j: u64 = (j.read());
        let v: Value<std_bitset_32_> = Rc::new(RefCell::new(std_bitset_32_::new_1({ j })));
        assert!(({ (j == ({ (*v.borrow()).to_ullong() })) }));
    }
    {
        let mut val: u64 = 6172934255747443285_u64;
        let mut canFit: bool =
            ((32_u64 as usize) < (::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 32_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<std_bitset_32_> = Rc::new(RefCell::new(std_bitset_32_::new_1({ val })));
        assert!(({ (({ (*v.borrow()).to_ullong() }) == (val & mask)) }));
    }
}
pub fn test_to_ullong_6() {
    let mut M: usize =
        (if ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) < (33_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            33_u64
        } as usize);
    let mut is_M_zero: bool = value_3.with(|rc| *rc.borrow());
    let mut X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: Value<u64> = Rc::new(RefCell::new(if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> X)
    }));
    let tests: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        0_u64,
        {
            let __tmp_24: Value<u64> = Rc::new(RefCell::new(1_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_24.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_24.as_pointer()
            }
            .read())
        },
        {
            let __tmp_25: Value<u64> = Rc::new(RefCell::new(2_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_25.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_25.as_pointer()
            }
            .read())
        },
        {
            let __tmp_26: Value<u64> = Rc::new(RefCell::new(3_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_26.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_26.as_pointer()
            }
            .read())
        },
        {
            let __tmp_27: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(3_u64)));
            (if CppLess::less(&__tmp_27.as_pointer(), &max.as_pointer()) {
                __tmp_27.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        {
            let __tmp_28: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(2_u64)));
            (if CppLess::less(&__tmp_28.as_pointer(), &max.as_pointer()) {
                __tmp_28.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        {
            let __tmp_29: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(1_u64)));
            (if CppLess::less(&__tmp_29.as_pointer(), &max.as_pointer()) {
                __tmp_29.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<u64>),
        (tests.as_pointer() as Ptr<u64>).offset((8_i64) as isize),
    ) {
        let mut j: u64 = (j.read());
        let v: Value<std_bitset_33_> = Rc::new(RefCell::new(std_bitset_33_::new_1({ j })));
        assert!(({ (j == ({ (*v.borrow()).to_ullong() })) }));
    }
    {
        let mut val: u64 = 6172934255747443285_u64;
        let mut canFit: bool =
            ((33_u64 as usize) < (::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 33_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<std_bitset_33_> = Rc::new(RefCell::new(std_bitset_33_::new_1({ val })));
        assert!(({ (({ (*v.borrow()).to_ullong() }) == (val & mask)) }));
    }
}
pub fn test_to_ullong_7() {
    let mut M: usize =
        (if ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) < (63_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            63_u64
        } as usize);
    let mut is_M_zero: bool = value_3.with(|rc| *rc.borrow());
    let mut X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: Value<u64> = Rc::new(RefCell::new(if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> X)
    }));
    let tests: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        0_u64,
        {
            let __tmp_30: Value<u64> = Rc::new(RefCell::new(1_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_30.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_30.as_pointer()
            }
            .read())
        },
        {
            let __tmp_31: Value<u64> = Rc::new(RefCell::new(2_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_31.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_31.as_pointer()
            }
            .read())
        },
        {
            let __tmp_32: Value<u64> = Rc::new(RefCell::new(3_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_32.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_32.as_pointer()
            }
            .read())
        },
        {
            let __tmp_33: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(3_u64)));
            (if CppLess::less(&__tmp_33.as_pointer(), &max.as_pointer()) {
                __tmp_33.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        {
            let __tmp_34: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(2_u64)));
            (if CppLess::less(&__tmp_34.as_pointer(), &max.as_pointer()) {
                __tmp_34.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        {
            let __tmp_35: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(1_u64)));
            (if CppLess::less(&__tmp_35.as_pointer(), &max.as_pointer()) {
                __tmp_35.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<u64>),
        (tests.as_pointer() as Ptr<u64>).offset((8_i64) as isize),
    ) {
        let mut j: u64 = (j.read());
        let v: Value<std_bitset_63_> = Rc::new(RefCell::new(std_bitset_63_::new_1({ j })));
        assert!(({ (j == ({ (*v.borrow()).to_ullong() })) }));
    }
    {
        let mut val: u64 = 6172934255747443285_u64;
        let mut canFit: bool =
            ((63_u64 as usize) < (::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 63_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<std_bitset_63_> = Rc::new(RefCell::new(std_bitset_63_::new_1({ val })));
        assert!(({ (({ (*v.borrow()).to_ullong() }) == (val & mask)) }));
    }
}
pub fn test_to_ullong_8() {
    let mut M: usize =
        (if ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) < (64_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            64_u64
        } as usize);
    let mut is_M_zero: bool = value_3.with(|rc| *rc.borrow());
    let mut X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: Value<u64> = Rc::new(RefCell::new(if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> X)
    }));
    let tests: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        0_u64,
        {
            let __tmp_36: Value<u64> = Rc::new(RefCell::new(1_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_36.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_36.as_pointer()
            }
            .read())
        },
        {
            let __tmp_37: Value<u64> = Rc::new(RefCell::new(2_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_37.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_37.as_pointer()
            }
            .read())
        },
        {
            let __tmp_38: Value<u64> = Rc::new(RefCell::new(3_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_38.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_38.as_pointer()
            }
            .read())
        },
        {
            let __tmp_39: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(3_u64)));
            (if CppLess::less(&__tmp_39.as_pointer(), &max.as_pointer()) {
                __tmp_39.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        {
            let __tmp_40: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(2_u64)));
            (if CppLess::less(&__tmp_40.as_pointer(), &max.as_pointer()) {
                __tmp_40.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        {
            let __tmp_41: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(1_u64)));
            (if CppLess::less(&__tmp_41.as_pointer(), &max.as_pointer()) {
                __tmp_41.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<u64>),
        (tests.as_pointer() as Ptr<u64>).offset((8_i64) as isize),
    ) {
        let mut j: u64 = (j.read());
        let v: Value<std_bitset_64_> = Rc::new(RefCell::new(std_bitset_64_::new_1({ j })));
        assert!(({ (j == ({ (*v.borrow()).to_ullong() })) }));
    }
    {
        let mut val: u64 = 6172934255747443285_u64;
        let mut canFit: bool =
            ((64_u64 as usize) < (::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 64_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<std_bitset_64_> = Rc::new(RefCell::new(std_bitset_64_::new_1({ val })));
        assert!(({ (({ (*v.borrow()).to_ullong() }) == (val & mask)) }));
    }
}
pub fn test_to_ullong_9() {
    let mut M: usize =
        (if ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) < (65_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            65_u64
        } as usize);
    let mut is_M_zero: bool = value_3.with(|rc| *rc.borrow());
    let mut X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: Value<u64> = Rc::new(RefCell::new(if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> X)
    }));
    let tests: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        0_u64,
        {
            let __tmp_42: Value<u64> = Rc::new(RefCell::new(1_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_42.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_42.as_pointer()
            }
            .read())
        },
        {
            let __tmp_43: Value<u64> = Rc::new(RefCell::new(2_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_43.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_43.as_pointer()
            }
            .read())
        },
        {
            let __tmp_44: Value<u64> = Rc::new(RefCell::new(3_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_44.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_44.as_pointer()
            }
            .read())
        },
        {
            let __tmp_45: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(3_u64)));
            (if CppLess::less(&__tmp_45.as_pointer(), &max.as_pointer()) {
                __tmp_45.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        {
            let __tmp_46: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(2_u64)));
            (if CppLess::less(&__tmp_46.as_pointer(), &max.as_pointer()) {
                __tmp_46.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        {
            let __tmp_47: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(1_u64)));
            (if CppLess::less(&__tmp_47.as_pointer(), &max.as_pointer()) {
                __tmp_47.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<u64>),
        (tests.as_pointer() as Ptr<u64>).offset((8_i64) as isize),
    ) {
        let mut j: u64 = (j.read());
        let v: Value<std_bitset_65_> = Rc::new(RefCell::new(std_bitset_65_::new_1({ j })));
        assert!(({ (j == ({ (*v.borrow()).to_ullong() })) }));
    }
    {
        let mut val: u64 = 6172934255747443285_u64;
        let mut canFit: bool =
            ((65_u64 as usize) < (::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 65_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<std_bitset_65_> = Rc::new(RefCell::new(std_bitset_65_::new_1({ val })));
        assert!(({ (({ (*v.borrow()).to_ullong() }) == (val & mask)) }));
    }
}
pub fn test_to_ullong_10() {
    let mut M: usize =
        (if ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) < (1000_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            1000_u64
        } as usize);
    let mut is_M_zero: bool = value_3.with(|rc| *rc.borrow());
    let mut X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: Value<u64> = Rc::new(RefCell::new(if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> X)
    }));
    let tests: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        0_u64,
        {
            let __tmp_48: Value<u64> = Rc::new(RefCell::new(1_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_48.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_48.as_pointer()
            }
            .read())
        },
        {
            let __tmp_49: Value<u64> = Rc::new(RefCell::new(2_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_49.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_49.as_pointer()
            }
            .read())
        },
        {
            let __tmp_50: Value<u64> = Rc::new(RefCell::new(3_u64));
            (if CppLess::less(&max.as_pointer(), &__tmp_50.as_pointer()) {
                max.as_pointer()
            } else {
                __tmp_50.as_pointer()
            }
            .read())
        },
        {
            let __tmp_51: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(3_u64)));
            (if CppLess::less(&__tmp_51.as_pointer(), &max.as_pointer()) {
                __tmp_51.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        {
            let __tmp_52: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(2_u64)));
            (if CppLess::less(&__tmp_52.as_pointer(), &max.as_pointer()) {
                __tmp_52.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        {
            let __tmp_53: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(1_u64)));
            (if CppLess::less(&__tmp_53.as_pointer(), &max.as_pointer()) {
                __tmp_53.as_pointer()
            } else {
                max.as_pointer()
            }
            .read())
        },
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<u64>),
        (tests.as_pointer() as Ptr<u64>).offset((8_i64) as isize),
    ) {
        let mut j: u64 = (j.read());
        let v: Value<std_bitset_1000_> = Rc::new(RefCell::new(std_bitset_1000_::new_1({ j })));
        assert!(({ (j == ({ (*v.borrow()).to_ullong() })) }));
    }
    {
        let mut val: u64 = 6172934255747443285_u64;
        let mut canFit: bool =
            ((1000_u64 as usize) < (::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 1000_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<std_bitset_1000_> = Rc::new(RefCell::new(std_bitset_1000_::new_1({ val })));
        assert!(({ (({ (*v.borrow()).to_ullong() }) == (val & mask)) }));
    }
}
pub fn test_11() -> bool {
    ({ test_to_ullong_0() });
    ({ test_to_ullong_2() });
    ({ test_to_ullong_4() });
    ({ test_to_ullong_5() });
    ({ test_to_ullong_6() });
    ({ test_to_ullong_7() });
    ({ test_to_ullong_8() });
    ({ test_to_ullong_9() });
    ({ test_to_ullong_10() });
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
    ({ test_11() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
