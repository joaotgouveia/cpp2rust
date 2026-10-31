extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_to_ullong_0() {
    let M: usize =
        (if (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize)) < (0_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            0_u64
        } as usize);
    let is_M_zero: bool = unimplemented!();
    let X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: u64 = if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> (X))
    };
    let mut tests: [u64; 8] = [
        0_u64,
        {
            let mut __tmp_0: u64 = 1_u64;
            (*{
                let __a: *const _ = &mut __tmp_0;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_1: u64 = 2_u64;
            (*{
                let __a: *const _ = &mut __tmp_1;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_2: u64 = 3_u64;
            (*{
                let __a: *const _ = &mut __tmp_2;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_3: u64 = (max).wrapping_sub(3_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_3;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_4: u64 = (max).wrapping_sub(2_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_4;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_5: u64 = (max).wrapping_sub(1_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_5;
                if *__b < *__a { __b } else { __a }
            })
        },
        max,
    ];
    'loop_: for j in IterRange::new(
        tests.as_mut_ptr(),
        tests.as_mut_ptr().offset((8_i64) as isize),
    ) {
        let mut j: u64 = (*j);
        let mut v: () = unimplemented!();
        assert!(((j) == (unimplemented!())));
    }
    {
        let val: u64 = 6172934255747443285_u64;
        let canFit: bool =
            ((0_u64 as usize) < ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize)));
        let mask: u64 = if canFit {
            (((1_u64) << (if canFit { 0_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let mut v: () = unimplemented!();
        assert!(((unimplemented!()) == ((val) & (mask))));
    }
}
pub unsafe fn test_to_ullong_1() {
    let M: usize =
        (if (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize)) < (1_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            1_u64
        } as usize);
    let is_M_zero: bool = unimplemented!();
    let X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: u64 = if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> (X))
    };
    let mut tests: [u64; 8] = [
        0_u64,
        {
            let mut __tmp_6: u64 = 1_u64;
            (*{
                let __a: *const _ = &mut __tmp_6;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_7: u64 = 2_u64;
            (*{
                let __a: *const _ = &mut __tmp_7;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_8: u64 = 3_u64;
            (*{
                let __a: *const _ = &mut __tmp_8;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_9: u64 = (max).wrapping_sub(3_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_9;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_10: u64 = (max).wrapping_sub(2_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_10;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_11: u64 = (max).wrapping_sub(1_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_11;
                if *__b < *__a { __b } else { __a }
            })
        },
        max,
    ];
    'loop_: for j in IterRange::new(
        tests.as_mut_ptr(),
        tests.as_mut_ptr().offset((8_i64) as isize),
    ) {
        let mut j: u64 = (*j);
        let mut v: () = unimplemented!();
        assert!(((j) == (unimplemented!())));
    }
    {
        let val: u64 = 6172934255747443285_u64;
        let canFit: bool =
            ((1_u64 as usize) < ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize)));
        let mask: u64 = if canFit {
            (((1_u64) << (if canFit { 1_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let mut v: () = unimplemented!();
        assert!(((unimplemented!()) == ((val) & (mask))));
    }
}
pub unsafe fn test_to_ullong_2() {
    let M: usize =
        (if (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize)) < (31_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            31_u64
        } as usize);
    let is_M_zero: bool = unimplemented!();
    let X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: u64 = if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> (X))
    };
    let mut tests: [u64; 8] = [
        0_u64,
        {
            let mut __tmp_12: u64 = 1_u64;
            (*{
                let __a: *const _ = &mut __tmp_12;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_13: u64 = 2_u64;
            (*{
                let __a: *const _ = &mut __tmp_13;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_14: u64 = 3_u64;
            (*{
                let __a: *const _ = &mut __tmp_14;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_15: u64 = (max).wrapping_sub(3_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_15;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_16: u64 = (max).wrapping_sub(2_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_16;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_17: u64 = (max).wrapping_sub(1_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_17;
                if *__b < *__a { __b } else { __a }
            })
        },
        max,
    ];
    'loop_: for j in IterRange::new(
        tests.as_mut_ptr(),
        tests.as_mut_ptr().offset((8_i64) as isize),
    ) {
        let mut j: u64 = (*j);
        let mut v: () = unimplemented!();
        assert!(((j) == (unimplemented!())));
    }
    {
        let val: u64 = 6172934255747443285_u64;
        let canFit: bool =
            ((31_u64 as usize) < ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize)));
        let mask: u64 = if canFit {
            (((1_u64) << (if canFit { 31_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let mut v: () = unimplemented!();
        assert!(((unimplemented!()) == ((val) & (mask))));
    }
}
pub unsafe fn test_to_ullong_3() {
    let M: usize =
        (if (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize)) < (32_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            32_u64
        } as usize);
    let is_M_zero: bool = unimplemented!();
    let X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: u64 = if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> (X))
    };
    let mut tests: [u64; 8] = [
        0_u64,
        {
            let mut __tmp_18: u64 = 1_u64;
            (*{
                let __a: *const _ = &mut __tmp_18;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_19: u64 = 2_u64;
            (*{
                let __a: *const _ = &mut __tmp_19;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_20: u64 = 3_u64;
            (*{
                let __a: *const _ = &mut __tmp_20;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_21: u64 = (max).wrapping_sub(3_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_21;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_22: u64 = (max).wrapping_sub(2_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_22;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_23: u64 = (max).wrapping_sub(1_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_23;
                if *__b < *__a { __b } else { __a }
            })
        },
        max,
    ];
    'loop_: for j in IterRange::new(
        tests.as_mut_ptr(),
        tests.as_mut_ptr().offset((8_i64) as isize),
    ) {
        let mut j: u64 = (*j);
        let mut v: () = unimplemented!();
        assert!(((j) == (unimplemented!())));
    }
    {
        let val: u64 = 6172934255747443285_u64;
        let canFit: bool =
            ((32_u64 as usize) < ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize)));
        let mask: u64 = if canFit {
            (((1_u64) << (if canFit { 32_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let mut v: () = unimplemented!();
        assert!(((unimplemented!()) == ((val) & (mask))));
    }
}
pub unsafe fn test_to_ullong_4() {
    let M: usize =
        (if (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize)) < (33_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            33_u64
        } as usize);
    let is_M_zero: bool = unimplemented!();
    let X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: u64 = if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> (X))
    };
    let mut tests: [u64; 8] = [
        0_u64,
        {
            let mut __tmp_24: u64 = 1_u64;
            (*{
                let __a: *const _ = &mut __tmp_24;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_25: u64 = 2_u64;
            (*{
                let __a: *const _ = &mut __tmp_25;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_26: u64 = 3_u64;
            (*{
                let __a: *const _ = &mut __tmp_26;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_27: u64 = (max).wrapping_sub(3_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_27;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_28: u64 = (max).wrapping_sub(2_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_28;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_29: u64 = (max).wrapping_sub(1_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_29;
                if *__b < *__a { __b } else { __a }
            })
        },
        max,
    ];
    'loop_: for j in IterRange::new(
        tests.as_mut_ptr(),
        tests.as_mut_ptr().offset((8_i64) as isize),
    ) {
        let mut j: u64 = (*j);
        let mut v: () = unimplemented!();
        assert!(((j) == (unimplemented!())));
    }
    {
        let val: u64 = 6172934255747443285_u64;
        let canFit: bool =
            ((33_u64 as usize) < ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize)));
        let mask: u64 = if canFit {
            (((1_u64) << (if canFit { 33_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let mut v: () = unimplemented!();
        assert!(((unimplemented!()) == ((val) & (mask))));
    }
}
pub unsafe fn test_to_ullong_5() {
    let M: usize =
        (if (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize)) < (63_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            63_u64
        } as usize);
    let is_M_zero: bool = unimplemented!();
    let X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: u64 = if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> (X))
    };
    let mut tests: [u64; 8] = [
        0_u64,
        {
            let mut __tmp_30: u64 = 1_u64;
            (*{
                let __a: *const _ = &mut __tmp_30;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_31: u64 = 2_u64;
            (*{
                let __a: *const _ = &mut __tmp_31;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_32: u64 = 3_u64;
            (*{
                let __a: *const _ = &mut __tmp_32;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_33: u64 = (max).wrapping_sub(3_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_33;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_34: u64 = (max).wrapping_sub(2_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_34;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_35: u64 = (max).wrapping_sub(1_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_35;
                if *__b < *__a { __b } else { __a }
            })
        },
        max,
    ];
    'loop_: for j in IterRange::new(
        tests.as_mut_ptr(),
        tests.as_mut_ptr().offset((8_i64) as isize),
    ) {
        let mut j: u64 = (*j);
        let mut v: () = unimplemented!();
        assert!(((j) == (unimplemented!())));
    }
    {
        let val: u64 = 6172934255747443285_u64;
        let canFit: bool =
            ((63_u64 as usize) < ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize)));
        let mask: u64 = if canFit {
            (((1_u64) << (if canFit { 63_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let mut v: () = unimplemented!();
        assert!(((unimplemented!()) == ((val) & (mask))));
    }
}
pub unsafe fn test_to_ullong_6() {
    let M: usize =
        (if (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize)) < (64_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            64_u64
        } as usize);
    let is_M_zero: bool = unimplemented!();
    let X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: u64 = if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> (X))
    };
    let mut tests: [u64; 8] = [
        0_u64,
        {
            let mut __tmp_36: u64 = 1_u64;
            (*{
                let __a: *const _ = &mut __tmp_36;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_37: u64 = 2_u64;
            (*{
                let __a: *const _ = &mut __tmp_37;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_38: u64 = 3_u64;
            (*{
                let __a: *const _ = &mut __tmp_38;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_39: u64 = (max).wrapping_sub(3_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_39;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_40: u64 = (max).wrapping_sub(2_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_40;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_41: u64 = (max).wrapping_sub(1_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_41;
                if *__b < *__a { __b } else { __a }
            })
        },
        max,
    ];
    'loop_: for j in IterRange::new(
        tests.as_mut_ptr(),
        tests.as_mut_ptr().offset((8_i64) as isize),
    ) {
        let mut j: u64 = (*j);
        let mut v: () = unimplemented!();
        assert!(((j) == (unimplemented!())));
    }
    {
        let val: u64 = 6172934255747443285_u64;
        let canFit: bool =
            ((64_u64 as usize) < ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize)));
        let mask: u64 = if canFit {
            (((1_u64) << (if canFit { 64_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let mut v: () = unimplemented!();
        assert!(((unimplemented!()) == ((val) & (mask))));
    }
}
pub unsafe fn test_to_ullong_7() {
    let M: usize =
        (if (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize)) < (65_u64 as usize)) {
            ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
        } else {
            65_u64
        } as usize);
    let is_M_zero: bool = unimplemented!();
    let X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: u64 = if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> (X))
    };
    let mut tests: [u64; 8] = [
        0_u64,
        {
            let mut __tmp_42: u64 = 1_u64;
            (*{
                let __a: *const _ = &mut __tmp_42;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_43: u64 = 2_u64;
            (*{
                let __a: *const _ = &mut __tmp_43;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_44: u64 = 3_u64;
            (*{
                let __a: *const _ = &mut __tmp_44;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_45: u64 = (max).wrapping_sub(3_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_45;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_46: u64 = (max).wrapping_sub(2_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_46;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_47: u64 = (max).wrapping_sub(1_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_47;
                if *__b < *__a { __b } else { __a }
            })
        },
        max,
    ];
    'loop_: for j in IterRange::new(
        tests.as_mut_ptr(),
        tests.as_mut_ptr().offset((8_i64) as isize),
    ) {
        let mut j: u64 = (*j);
        let mut v: () = unimplemented!();
        assert!(((j) == (unimplemented!())));
    }
    {
        let val: u64 = 6172934255747443285_u64;
        let canFit: bool =
            ((65_u64 as usize) < ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize)));
        let mask: u64 = if canFit {
            (((1_u64) << (if canFit { 65_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let mut v: () = unimplemented!();
        assert!(((unimplemented!()) == ((val) & (mask))));
    }
}
pub unsafe fn test_to_ullong_8() {
    let M: usize = (if (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize))
        < (1000_u64 as usize))
    {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
    } else {
        1000_u64
    } as usize);
    let is_M_zero: bool = unimplemented!();
    let X: usize = (if is_M_zero {
        (((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as usize)
            .wrapping_sub(1_usize) as u64)
    } else {
        ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize) as u64)
            .wrapping_sub((M as u64))
    } as usize);
    let max: u64 = if is_M_zero {
        0_u64
    } else {
        (((-1_i32) as u64) >> (X))
    };
    let mut tests: [u64; 8] = [
        0_u64,
        {
            let mut __tmp_48: u64 = 1_u64;
            (*{
                let __a: *const _ = &mut __tmp_48;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_49: u64 = 2_u64;
            (*{
                let __a: *const _ = &mut __tmp_49;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_50: u64 = 3_u64;
            (*{
                let __a: *const _ = &mut __tmp_50;
                let __b: *const _ = &max;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_51: u64 = (max).wrapping_sub(3_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_51;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_52: u64 = (max).wrapping_sub(2_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_52;
                if *__b < *__a { __b } else { __a }
            })
        },
        {
            let mut __tmp_53: u64 = (max).wrapping_sub(1_u64);
            (*{
                let __a: *const _ = &max;
                let __b: *const _ = &mut __tmp_53;
                if *__b < *__a { __b } else { __a }
            })
        },
        max,
    ];
    'loop_: for j in IterRange::new(
        tests.as_mut_ptr(),
        tests.as_mut_ptr().offset((8_i64) as isize),
    ) {
        let mut j: u64 = (*j);
        let mut v: () = unimplemented!();
        assert!(((j) == (unimplemented!())));
    }
    {
        let val: u64 = 6172934255747443285_u64;
        let canFit: bool =
            ((1000_u64 as usize) < ((::std::mem::size_of::<u64>() as usize).wrapping_mul(8_usize)));
        let mask: u64 = if canFit {
            (((1_u64) << (if canFit { 1000_u64 } else { 0_u64 })) as u64).wrapping_sub(1_u64)
        } else {
            ((-1_i32) as u64)
        };
        let mut v: () = unimplemented!();
        assert!(((unimplemented!()) == ((val) & (mask))));
    }
}
pub unsafe fn test_9() -> bool {
    (unsafe { test_to_ullong_0() });
    (unsafe { test_to_ullong_1() });
    (unsafe { test_to_ullong_2() });
    (unsafe { test_to_ullong_3() });
    (unsafe { test_to_ullong_4() });
    (unsafe { test_to_ullong_5() });
    (unsafe { test_to_ullong_6() });
    (unsafe { test_to_ullong_7() });
    (unsafe { test_to_ullong_8() });
    return true;
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_9() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
