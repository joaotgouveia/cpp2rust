extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_char_pointer_ctor_0() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_0: u64 = (unimplemented!() as u64);
            let mut __tmp_1: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_0;
                let __b: *const _ = &mut __tmp_1;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_2: u64 = (unimplemented!() as u64);
            let mut __tmp_3: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_2;
                let __b: *const _ = &mut __tmp_3;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1a1a1a1a1a\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_4: u64 = (unimplemented!() as u64);
            let mut __tmp_5: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_4;
                let __b: *const _ = &mut __tmp_5;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"bababababa\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_6: u64 = (unimplemented!() as u64);
            let mut __tmp_7: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_6;
                let __b: *const _ = &mut __tmp_7;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('b' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_8: u64 = (unimplemented!() as u64);
            let mut __tmp_9: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_8;
                let __b: *const _ = &mut __tmp_9;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_10: u64 = (unimplemented!() as u64);
            let mut __tmp_11: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_10;
                let __b: *const _ = &mut __tmp_11;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_12: u64 = (unimplemented!() as u64);
            let mut __tmp_13: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_12;
                let __b: *const _ = &mut __tmp_13;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('b' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_char_pointer_ctor_1() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_14: u64 = (unimplemented!() as u64);
            let mut __tmp_15: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_14;
                let __b: *const _ = &mut __tmp_15;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_16: u64 = (unimplemented!() as u64);
            let mut __tmp_17: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_16;
                let __b: *const _ = &mut __tmp_17;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1a1a1a1a1a\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_18: u64 = (unimplemented!() as u64);
            let mut __tmp_19: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_18;
                let __b: *const _ = &mut __tmp_19;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"bababababa\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_20: u64 = (unimplemented!() as u64);
            let mut __tmp_21: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_20;
                let __b: *const _ = &mut __tmp_21;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('b' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_22: u64 = (unimplemented!() as u64);
            let mut __tmp_23: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_22;
                let __b: *const _ = &mut __tmp_23;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_24: u64 = (unimplemented!() as u64);
            let mut __tmp_25: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_24;
                let __b: *const _ = &mut __tmp_25;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_26: u64 = (unimplemented!() as u64);
            let mut __tmp_27: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_26;
                let __b: *const _ = &mut __tmp_27;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('b' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_char_pointer_ctor_2() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_28: u64 = (unimplemented!() as u64);
            let mut __tmp_29: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_28;
                let __b: *const _ = &mut __tmp_29;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_30: u64 = (unimplemented!() as u64);
            let mut __tmp_31: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_30;
                let __b: *const _ = &mut __tmp_31;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1a1a1a1a1a\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_32: u64 = (unimplemented!() as u64);
            let mut __tmp_33: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_32;
                let __b: *const _ = &mut __tmp_33;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"bababababa\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_34: u64 = (unimplemented!() as u64);
            let mut __tmp_35: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_34;
                let __b: *const _ = &mut __tmp_35;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('b' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_36: u64 = (unimplemented!() as u64);
            let mut __tmp_37: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_36;
                let __b: *const _ = &mut __tmp_37;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_38: u64 = (unimplemented!() as u64);
            let mut __tmp_39: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_38;
                let __b: *const _ = &mut __tmp_39;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_40: u64 = (unimplemented!() as u64);
            let mut __tmp_41: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_40;
                let __b: *const _ = &mut __tmp_41;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('b' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_char_pointer_ctor_3() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_42: u64 = (unimplemented!() as u64);
            let mut __tmp_43: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_42;
                let __b: *const _ = &mut __tmp_43;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_44: u64 = (unimplemented!() as u64);
            let mut __tmp_45: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_44;
                let __b: *const _ = &mut __tmp_45;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1a1a1a1a1a\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_46: u64 = (unimplemented!() as u64);
            let mut __tmp_47: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_46;
                let __b: *const _ = &mut __tmp_47;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"bababababa\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_48: u64 = (unimplemented!() as u64);
            let mut __tmp_49: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_48;
                let __b: *const _ = &mut __tmp_49;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('b' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_50: u64 = (unimplemented!() as u64);
            let mut __tmp_51: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_50;
                let __b: *const _ = &mut __tmp_51;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_52: u64 = (unimplemented!() as u64);
            let mut __tmp_53: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_52;
                let __b: *const _ = &mut __tmp_53;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_54: u64 = (unimplemented!() as u64);
            let mut __tmp_55: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_54;
                let __b: *const _ = &mut __tmp_55;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('b' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_char_pointer_ctor_4() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_56: u64 = (unimplemented!() as u64);
            let mut __tmp_57: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_56;
                let __b: *const _ = &mut __tmp_57;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_58: u64 = (unimplemented!() as u64);
            let mut __tmp_59: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_58;
                let __b: *const _ = &mut __tmp_59;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1a1a1a1a1a\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_60: u64 = (unimplemented!() as u64);
            let mut __tmp_61: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_60;
                let __b: *const _ = &mut __tmp_61;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"bababababa\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_62: u64 = (unimplemented!() as u64);
            let mut __tmp_63: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_62;
                let __b: *const _ = &mut __tmp_63;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('b' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_64: u64 = (unimplemented!() as u64);
            let mut __tmp_65: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_64;
                let __b: *const _ = &mut __tmp_65;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_66: u64 = (unimplemented!() as u64);
            let mut __tmp_67: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_66;
                let __b: *const _ = &mut __tmp_67;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_68: u64 = (unimplemented!() as u64);
            let mut __tmp_69: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_68;
                let __b: *const _ = &mut __tmp_69;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('b' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_char_pointer_ctor_5() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_70: u64 = (unimplemented!() as u64);
            let mut __tmp_71: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_70;
                let __b: *const _ = &mut __tmp_71;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_72: u64 = (unimplemented!() as u64);
            let mut __tmp_73: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_72;
                let __b: *const _ = &mut __tmp_73;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1a1a1a1a1a\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_74: u64 = (unimplemented!() as u64);
            let mut __tmp_75: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_74;
                let __b: *const _ = &mut __tmp_75;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"bababababa\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_76: u64 = (unimplemented!() as u64);
            let mut __tmp_77: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_76;
                let __b: *const _ = &mut __tmp_77;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('b' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_78: u64 = (unimplemented!() as u64);
            let mut __tmp_79: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_78;
                let __b: *const _ = &mut __tmp_79;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_80: u64 = (unimplemented!() as u64);
            let mut __tmp_81: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_80;
                let __b: *const _ = &mut __tmp_81;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_82: u64 = (unimplemented!() as u64);
            let mut __tmp_83: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_82;
                let __b: *const _ = &mut __tmp_83;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('b' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_char_pointer_ctor_6() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_84: u64 = (unimplemented!() as u64);
            let mut __tmp_85: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_84;
                let __b: *const _ = &mut __tmp_85;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_86: u64 = (unimplemented!() as u64);
            let mut __tmp_87: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_86;
                let __b: *const _ = &mut __tmp_87;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1a1a1a1a1a\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_88: u64 = (unimplemented!() as u64);
            let mut __tmp_89: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_88;
                let __b: *const _ = &mut __tmp_89;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"bababababa\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_90: u64 = (unimplemented!() as u64);
            let mut __tmp_91: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_90;
                let __b: *const _ = &mut __tmp_91;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('b' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_92: u64 = (unimplemented!() as u64);
            let mut __tmp_93: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_92;
                let __b: *const _ = &mut __tmp_93;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_94: u64 = (unimplemented!() as u64);
            let mut __tmp_95: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_94;
                let __b: *const _ = &mut __tmp_95;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_96: u64 = (unimplemented!() as u64);
            let mut __tmp_97: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_96;
                let __b: *const _ = &mut __tmp_97;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('b' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_char_pointer_ctor_7() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_98: u64 = (unimplemented!() as u64);
            let mut __tmp_99: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_98;
                let __b: *const _ = &mut __tmp_99;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_100: u64 = (unimplemented!() as u64);
            let mut __tmp_101: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_100;
                let __b: *const _ = &mut __tmp_101;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1a1a1a1a1a\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_102: u64 = (unimplemented!() as u64);
            let mut __tmp_103: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_102;
                let __b: *const _ = &mut __tmp_103;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"bababababa\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_104: u64 = (unimplemented!() as u64);
            let mut __tmp_105: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_104;
                let __b: *const _ = &mut __tmp_105;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('b' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_106: u64 = (unimplemented!() as u64);
            let mut __tmp_107: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_106;
                let __b: *const _ = &mut __tmp_107;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_108: u64 = (unimplemented!() as u64);
            let mut __tmp_109: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_108;
                let __b: *const _ = &mut __tmp_109;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_110: u64 = (unimplemented!() as u64);
            let mut __tmp_111: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_110;
                let __b: *const _ = &mut __tmp_111;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('b' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_char_pointer_ctor_8() {
    const _: () = assert!(
        true,
        "!std::is_convertible<const char*, std::bitset<N> >::value"
    );;
    const _: () = assert!(
        true,
        "std::is_constructible<std::bitset<N>, const char*>::value"
    );;
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_112: u64 = (unimplemented!() as u64);
            let mut __tmp_113: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_112;
                let __b: *const _ = &mut __tmp_113;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1010101010\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_114: u64 = (unimplemented!() as u64);
            let mut __tmp_115: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_114;
                let __b: *const _ = &mut __tmp_115;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"1a1a1a1a1a\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_116: u64 = (unimplemented!() as u64);
            let mut __tmp_117: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_116;
                let __b: *const _ = &mut __tmp_117;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let s: [libc::c_char; 11] = std::mem::transmute(*b"bababababa\0");
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_118: u64 = (unimplemented!() as u64);
            let mut __tmp_119: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_118;
                let __b: *const _ = &mut __tmp_119;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((s[(((M).wrapping_sub(1_usize)).wrapping_sub(i))] as i32)
                        == (('b' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(((unimplemented!() as i32) == (false as i32)));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
            ('1' as libc::c_char),
            ('0' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_120: u64 = (unimplemented!() as u64);
            let mut __tmp_121: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_120;
                let __b: *const _ = &mut __tmp_121;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
            ('1' as libc::c_char),
            ('a' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_122: u64 = (unimplemented!() as u64);
            let mut __tmp_123: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_122;
                let __b: *const _ = &mut __tmp_123;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('1' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
    {
        let not_null_terminated: [libc::c_char; 10] = [
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
            ('b' as libc::c_char),
            ('a' as libc::c_char),
        ];
        let mut v: () = unimplemented!();
        let mut M: usize = ({
            let mut __tmp_124: u64 = (unimplemented!() as u64);
            let mut __tmp_125: u64 = 10_u64;
            (*{
                let __a: *const _ = &mut __tmp_124;
                let __b: *const _ = &mut __tmp_125;
                if *__b < *__a { __b } else { __a }
            })
        } as usize);
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (M)) {
            assert!(
                ((unimplemented!() as i32)
                    == (((not_null_terminated[(((M).wrapping_sub(1_usize)).wrapping_sub(i))]
                        as i32)
                        == (('b' as libc::c_char) as i32)) as i32))
            );
            i.prefix_inc();
        }
        let mut i: usize = 10_usize;
        'loop_: while ((i) < (unimplemented!())) {
            assert!(!(unimplemented!()));
            i.prefix_inc();
        }
    }
}
pub unsafe fn test_9() -> bool {
    (unsafe { test_char_pointer_ctor_0() });
    (unsafe { test_char_pointer_ctor_1() });
    (unsafe { test_char_pointer_ctor_2() });
    (unsafe { test_char_pointer_ctor_3() });
    (unsafe { test_char_pointer_ctor_4() });
    (unsafe { test_char_pointer_ctor_5() });
    (unsafe { test_char_pointer_ctor_6() });
    (unsafe { test_char_pointer_ctor_7() });
    (unsafe { test_char_pointer_ctor_8() });
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
