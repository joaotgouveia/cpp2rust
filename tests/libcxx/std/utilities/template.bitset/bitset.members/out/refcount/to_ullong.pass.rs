extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn test_to_ullong_0() {
    let mut M: u64 = if ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) < 0_u64) {
        ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64)
    } else {
        0_u64
    };
    let mut is_M_zero: bool = value_1.with(|rc| *rc.borrow());
    let mut X: u64 = if is_M_zero {
        (((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64).wrapping_sub(1_u64)
            as u64)
    } else {
        ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64).wrapping_sub(M)
    };
    let max: Value<u64> = Rc::new(RefCell::new(if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> X)
    }));
    let tests: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        0_u64,
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(1_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(2_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(3_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(3_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(2_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(1_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<u64>),
        (tests.as_pointer() as Ptr<u64>).offset((8_i64) as isize),
    ) {
        let mut j: u64 = (j.read());
        let v: Value<std_bitset_0_> = Rc::new(RefCell::new(std_bitset_0_::new_1({ j })));
        ({ cpp2rust_assert_fail_3((j == ({ (*v.borrow()).to_ullong() }))) });
    }
    {
        let mut val: u64 = 6172934255747443285_u64;
        let mut canFit: bool = (0_u64 < (::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 0_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<std_bitset_0_> = Rc::new(RefCell::new(std_bitset_0_::new_1({ val })));
        ({ cpp2rust_assert_fail_3((({ (*v.borrow()).to_ullong() }) == (val & mask))) });
    }
}
pub fn test_to_ullong_4() {
    let mut M: u64 = if ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) < 1_u64) {
        ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64)
    } else {
        1_u64
    };
    let mut is_M_zero: bool = value_5.with(|rc| *rc.borrow());
    let mut X: u64 = if is_M_zero {
        (((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64).wrapping_sub(1_u64)
            as u64)
    } else {
        ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64).wrapping_sub(M)
    };
    let max: Value<u64> = Rc::new(RefCell::new(if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> X)
    }));
    let tests: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        0_u64,
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(1_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(2_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(3_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(3_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(2_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(1_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<u64>),
        (tests.as_pointer() as Ptr<u64>).offset((8_i64) as isize),
    ) {
        let mut j: u64 = (j.read());
        let v: Value<std_bitset_1_> = Rc::new(RefCell::new(std_bitset_1_::new_1({ j })));
        ({ cpp2rust_assert_fail_3((j == ({ (*v.borrow()).to_ullong() }))) });
    }
    {
        let mut val: u64 = 6172934255747443285_u64;
        let mut canFit: bool = (1_u64 < (::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 1_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<std_bitset_1_> = Rc::new(RefCell::new(std_bitset_1_::new_1({ val })));
        ({ cpp2rust_assert_fail_3((({ (*v.borrow()).to_ullong() }) == (val & mask))) });
    }
}
pub fn test_to_ullong_6() {
    let mut M: u64 = if ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) < 31_u64) {
        ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64)
    } else {
        31_u64
    };
    let mut is_M_zero: bool = value_5.with(|rc| *rc.borrow());
    let mut X: u64 = if is_M_zero {
        (((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64).wrapping_sub(1_u64)
            as u64)
    } else {
        ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64).wrapping_sub(M)
    };
    let max: Value<u64> = Rc::new(RefCell::new(if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> X)
    }));
    let tests: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        0_u64,
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(1_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(2_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(3_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(3_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(2_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(1_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<u64>),
        (tests.as_pointer() as Ptr<u64>).offset((8_i64) as isize),
    ) {
        let mut j: u64 = (j.read());
        let v: Value<std_bitset_31_> = Rc::new(RefCell::new(std_bitset_31_::new_1({ j })));
        ({ cpp2rust_assert_fail_3((j == ({ (*v.borrow()).to_ullong() }))) });
    }
    {
        let mut val: u64 = 6172934255747443285_u64;
        let mut canFit: bool = (31_u64 < (::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 31_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<std_bitset_31_> = Rc::new(RefCell::new(std_bitset_31_::new_1({ val })));
        ({ cpp2rust_assert_fail_3((({ (*v.borrow()).to_ullong() }) == (val & mask))) });
    }
}
pub fn test_to_ullong_7() {
    let mut M: u64 = if ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) < 32_u64) {
        ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64)
    } else {
        32_u64
    };
    let mut is_M_zero: bool = value_5.with(|rc| *rc.borrow());
    let mut X: u64 = if is_M_zero {
        (((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64).wrapping_sub(1_u64)
            as u64)
    } else {
        ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64).wrapping_sub(M)
    };
    let max: Value<u64> = Rc::new(RefCell::new(if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> X)
    }));
    let tests: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        0_u64,
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(1_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(2_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(3_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(3_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(2_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(1_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<u64>),
        (tests.as_pointer() as Ptr<u64>).offset((8_i64) as isize),
    ) {
        let mut j: u64 = (j.read());
        let v: Value<std_bitset_32_> = Rc::new(RefCell::new(std_bitset_32_::new_1({ j })));
        ({ cpp2rust_assert_fail_3((j == ({ (*v.borrow()).to_ullong() }))) });
    }
    {
        let mut val: u64 = 6172934255747443285_u64;
        let mut canFit: bool = (32_u64 < (::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 32_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<std_bitset_32_> = Rc::new(RefCell::new(std_bitset_32_::new_1({ val })));
        ({ cpp2rust_assert_fail_3((({ (*v.borrow()).to_ullong() }) == (val & mask))) });
    }
}
pub fn test_to_ullong_8() {
    let mut M: u64 = if ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) < 33_u64) {
        ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64)
    } else {
        33_u64
    };
    let mut is_M_zero: bool = value_5.with(|rc| *rc.borrow());
    let mut X: u64 = if is_M_zero {
        (((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64).wrapping_sub(1_u64)
            as u64)
    } else {
        ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64).wrapping_sub(M)
    };
    let max: Value<u64> = Rc::new(RefCell::new(if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> X)
    }));
    let tests: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        0_u64,
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(1_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(2_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(3_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(3_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(2_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(1_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<u64>),
        (tests.as_pointer() as Ptr<u64>).offset((8_i64) as isize),
    ) {
        let mut j: u64 = (j.read());
        let v: Value<std_bitset_33_> = Rc::new(RefCell::new(std_bitset_33_::new_1({ j })));
        ({ cpp2rust_assert_fail_3((j == ({ (*v.borrow()).to_ullong() }))) });
    }
    {
        let mut val: u64 = 6172934255747443285_u64;
        let mut canFit: bool = (33_u64 < (::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 33_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<std_bitset_33_> = Rc::new(RefCell::new(std_bitset_33_::new_1({ val })));
        ({ cpp2rust_assert_fail_3((({ (*v.borrow()).to_ullong() }) == (val & mask))) });
    }
}
pub fn test_to_ullong_9() {
    let mut M: u64 = if ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) < 63_u64) {
        ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64)
    } else {
        63_u64
    };
    let mut is_M_zero: bool = value_5.with(|rc| *rc.borrow());
    let mut X: u64 = if is_M_zero {
        (((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64).wrapping_sub(1_u64)
            as u64)
    } else {
        ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64).wrapping_sub(M)
    };
    let max: Value<u64> = Rc::new(RefCell::new(if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> X)
    }));
    let tests: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        0_u64,
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(1_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(2_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(3_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(3_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(2_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(1_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<u64>),
        (tests.as_pointer() as Ptr<u64>).offset((8_i64) as isize),
    ) {
        let mut j: u64 = (j.read());
        let v: Value<std_bitset_63_> = Rc::new(RefCell::new(std_bitset_63_::new_1({ j })));
        ({ cpp2rust_assert_fail_3((j == ({ (*v.borrow()).to_ullong() }))) });
    }
    {
        let mut val: u64 = 6172934255747443285_u64;
        let mut canFit: bool = (63_u64 < (::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 63_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<std_bitset_63_> = Rc::new(RefCell::new(std_bitset_63_::new_1({ val })));
        ({ cpp2rust_assert_fail_3((({ (*v.borrow()).to_ullong() }) == (val & mask))) });
    }
}
pub fn test_to_ullong_10() {
    let mut M: u64 = if ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) < 64_u64) {
        ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64)
    } else {
        64_u64
    };
    let mut is_M_zero: bool = value_5.with(|rc| *rc.borrow());
    let mut X: u64 = if is_M_zero {
        (((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64).wrapping_sub(1_u64)
            as u64)
    } else {
        ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64).wrapping_sub(M)
    };
    let max: Value<u64> = Rc::new(RefCell::new(if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> X)
    }));
    let tests: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        0_u64,
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(1_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(2_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(3_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(3_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(2_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(1_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<u64>),
        (tests.as_pointer() as Ptr<u64>).offset((8_i64) as isize),
    ) {
        let mut j: u64 = (j.read());
        let v: Value<std_bitset_64_> = Rc::new(RefCell::new(std_bitset_64_::new_1({ j })));
        ({ cpp2rust_assert_fail_3((j == ({ (*v.borrow()).to_ullong() }))) });
    }
    {
        let mut val: u64 = 6172934255747443285_u64;
        let mut canFit: bool = (64_u64 < (::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 64_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<std_bitset_64_> = Rc::new(RefCell::new(std_bitset_64_::new_1({ val })));
        ({ cpp2rust_assert_fail_3((({ (*v.borrow()).to_ullong() }) == (val & mask))) });
    }
}
pub fn test_to_ullong_11() {
    let mut M: u64 = if ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) < 65_u64) {
        ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64)
    } else {
        65_u64
    };
    let mut is_M_zero: bool = value_5.with(|rc| *rc.borrow());
    let mut X: u64 = if is_M_zero {
        (((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64).wrapping_sub(1_u64)
            as u64)
    } else {
        ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64).wrapping_sub(M)
    };
    let max: Value<u64> = Rc::new(RefCell::new(if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> X)
    }));
    let tests: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        0_u64,
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(1_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(2_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(3_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(3_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(2_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(1_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<u64>),
        (tests.as_pointer() as Ptr<u64>).offset((8_i64) as isize),
    ) {
        let mut j: u64 = (j.read());
        let v: Value<std_bitset_65_> = Rc::new(RefCell::new(std_bitset_65_::new_1({ j })));
        ({ cpp2rust_assert_fail_3((j == ({ (*v.borrow()).to_ullong() }))) });
    }
    {
        let mut val: u64 = 6172934255747443285_u64;
        let mut canFit: bool = (65_u64 < (::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 65_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<std_bitset_65_> = Rc::new(RefCell::new(std_bitset_65_::new_1({ val })));
        ({ cpp2rust_assert_fail_3((({ (*v.borrow()).to_ullong() }) == (val & mask))) });
    }
}
pub fn test_to_ullong_12() {
    let mut M: u64 = if ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) < 1000_u64) {
        ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64)
    } else {
        1000_u64
    };
    let mut is_M_zero: bool = value_5.with(|rc| *rc.borrow());
    let mut X: u64 = if is_M_zero {
        (((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64).wrapping_sub(1_u64)
            as u64)
    } else {
        ((::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64) as u64).wrapping_sub(M)
    };
    let max: Value<u64> = Rc::new(RefCell::new(if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> X)
    }));
    let tests: Value<Box<[u64]>> = Rc::new(RefCell::new(Box::new([
        0_u64,
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(1_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(2_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Value<u64> = Rc::new(RefCell::new(3_u64));
            min_2(___a.as_pointer(), max.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(3_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(2_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (({
            let ___a: Ptr<u64> = max.as_pointer();
            let ___b: Value<u64> = Rc::new(RefCell::new((*max.borrow()).wrapping_sub(1_u64)));
            min_2(___a, ___b.as_pointer())
        })
        .read()),
        (*max.borrow()),
    ])));
    'loop_: for j in IterRange::new(
        (tests.as_pointer() as Ptr<u64>),
        (tests.as_pointer() as Ptr<u64>).offset((8_i64) as isize),
    ) {
        let mut j: u64 = (j.read());
        let v: Value<std_bitset_1000_> = Rc::new(RefCell::new(std_bitset_1000_::new_1({ j })));
        ({ cpp2rust_assert_fail_3((j == ({ (*v.borrow()).to_ullong() }))) });
    }
    {
        let mut val: u64 = 6172934255747443285_u64;
        let mut canFit: bool =
            (1000_u64 < (::std::mem::size_of::<u64>() as u64).wrapping_mul(8_u64));
        let mut mask: u64 = if canFit {
            ((1_u64 << (if canFit { 1000_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let v: Value<std_bitset_1000_> = Rc::new(RefCell::new(std_bitset_1000_::new_1({ val })));
        ({ cpp2rust_assert_fail_3((({ (*v.borrow()).to_ullong() }) == (val & mask))) });
    }
}
pub fn test_13() -> bool {
    ({ test_to_ullong_0() });
    ({ test_to_ullong_4() });
    ({ test_to_ullong_6() });
    ({ test_to_ullong_7() });
    ({ test_to_ullong_8() });
    ({ test_to_ullong_9() });
    ({ test_to_ullong_10() });
    ({ test_to_ullong_11() });
    ({ test_to_ullong_12() });
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
    ({ test_13() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
